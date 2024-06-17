use chrono::Utc;
use dotenv::dotenv;
use futures_lite::stream::StreamExt;
use lapin::{Connection, ConnectionProperties};
use lapin::message::Delivery;
use lapin::options::BasicAckOptions;
use log::{debug, info};
use reqwest::Client;
use serde::Deserialize;

use crate::config::Config;
use crate::zone::{add_record, get_zone_by_name, update_record_content};

mod config;
mod auth;
mod zone;

#[derive(Deserialize, Debug)]
struct RowData {
    hostname: String,
    ipv4: String,
}

async fn executor(delivery: &Delivery, config: &Config, client: &Client) {
    let values: Vec<RowData> = match serde_json::from_slice::<serde_json::Value>(&delivery.data) {
        Ok(data) => match serde_json::from_value::<Vec<RowData>>(data) {
            Ok(values) => values,
            Err(error) => {
                log::warn!("Failed to parse json: {}", error);
                match delivery.ack(BasicAckOptions::default()).await {
                    Ok(_) => (),
                    Err(error) => log::warn!("Failed to ack delivery: {}", error),
                };
                return;
            },
        },
        Err(error) => {
            log::warn!("Failed to parse json: {}", error);
            match delivery.ack(BasicAckOptions::default()).await {
                Ok(_) => (),
                Err(error) => log::warn!("Failed to ack delivery: {}", error),
            };
            return;
        }
    };

    let token = auth::make_token(
        &client,
        config.selectel.username.as_str(),
        config.selectel.account_id,
        config.selectel.password.as_str(),
        config.selectel.project.as_str(),
        config.selectel.auth_url.as_str()
    ).await;

    let zone_id = get_zone_by_name(
        &client,
        &config.selectel.zone_name.to_lowercase(),
        &config.selectel.zone_url,
        &token
    ).await;

    for value in values {
        let record_name = format!(
            "{}{}.{}",
            value.hostname,
            config.hostname_suffix,
            config.selectel.zone_name
        ).to_lowercase();

        let record = zone::get_record_by_name(
            &client,
            &record_name,
            &zone_id,
            &config.selectel.zone_url,
            &token
        ).await;

        if record.is_none() {
            add_record(
                &client,
                &zone_id,
                &record_name,
                &value.ipv4,
                "A",
                config.selectel.ttl,
                &config.selectel.zone_url,
                &token
            ).await;
            continue;
        }

        let (record_id, record_ip) = record.unwrap();

        if record_ip != value.ipv4 {
            update_record_content(
                &client,
                &zone_id,
                &record_id,
                &value.ipv4,
                config.selectel.ttl,
                &format!(
                    "Updated by dnsync at {} | {} => {}",
                    Utc::now(),
                    record_ip,
                    value.ipv4
                ),
                &config.selectel.zone_url,
                &token
            ).await;
        }
    }

    match delivery.ack(BasicAckOptions::default()).await {
        Ok(_) => log::info!("Successfully acked delivery"),
        Err(error) => log::warn!("Failed to ack delivery: {}", error),
    };
}


#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    // env_logger::builder()
    //     // .filter_module("consulrs", log::LevelFilter::Error)
    //     // .filter_module("tracing", log::LevelFilter::Error)
    //     // .filter_module("rustify", log::LevelFilter::Error)
    //     .init();
    

    let config = Config::new();
    let client = reqwest::Client::new();
    let addr = &format!(
        "amqp://{}:{}@{}:{}/{}",
        config.rabbitmq.user,
        config.rabbitmq.password,
        config.rabbitmq.host,
        config.rabbitmq.port,
        config.rabbitmq.vhost
    );
    
    let interval = std::time::Duration::from_secs(config.sync_interval as u64);
    
    loop {
        let rmq = match Connection::connect(
            addr,
            ConnectionProperties::default()
        ).await {
            Ok(rmq) => rmq,
            Err(error) => {
                log::error!("Failed to create connection: {}", error);
                std::process::exit(1);
            },
        };
        
        match rmq.create_channel().await {
            Ok(channel) => {
                let mut consumer = match channel
                    .basic_consume(
                        &config.rabbitmq.queue,
                        "my_consumer",
                        Default::default(),
                        Default::default(),
                    )
                    .await {
                    Ok(consumer) => consumer,
                    Err(error) => {
                        log::error!("Failed to create consumer: {}", error);
                        std::process::exit(1);
                    },
                };

                match consumer.next().await {
                    Some(delivery) => match delivery {
                        Ok(delivery) => executor(&delivery, &config, &client).await,
                        Err(error) => debug!("Failed to get delivery: {}", error)
                    },
                    None => {},
                }
            }
            Err(error) => {
                log::error!("Failed to create channel: {}", error);
            }
        }
        
        info!("Sleeping for {} seconds", config.sync_interval);
        rmq.close(0, "Developer of lapin lib is dolbaeb").await.ok();
        tokio::time::sleep(interval).await;
    }
}
