use log::error;

pub struct RabbitMQConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub vhost: String,
    pub queue: String,
}

pub struct SelectelConfig {
    pub project: String,
    pub username: String,
    pub password: String,
    pub account_id: u32,
    pub zone_name: String,
    pub auth_url: String,
    pub zone_url: String,
    pub ttl: u32,
}

pub struct Config {
    pub sync_interval: u32,
    pub hostname_suffix: String,
    pub rabbitmq: RabbitMQConfig,
    pub selectel: SelectelConfig,
}


fn get_var(name: &str) -> String {
    match std::env::var(name) {
        Ok(value) => value,
        Err(_) => {
            error!("{} is not set", name);
            std::process::exit(1);
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            sync_interval: get_var("SYNC_INTERVAL").parse().unwrap(),
            hostname_suffix: get_var("HOSTNAME_SUFFIX"),
            rabbitmq: RabbitMQConfig {
                host: get_var("RABBITMQ_HOST"),
                port: get_var("RABBITMQ_PORT").parse().unwrap(),
                user: get_var("RABBITMQ_USER"),
                password: get_var("RABBITMQ_PASSWORD"),
                vhost: get_var("RABBITMQ_VH"),
                queue: get_var("RABBITMQ_QUEUE"),
            },
            selectel: SelectelConfig {
                project: get_var("SELECTEL_PROJECT"),
                username: get_var("SELECTEL_USERNAME"),
                password: get_var("SELECTEL_PASSWORD"),
                account_id: get_var("SELECTEL_ACCOUNT_ID").parse().unwrap(),
                zone_name: get_var("SELECTEL_ZONE_NAME"),
                ttl: get_var("SELECTEL_RECORD_TTL").parse().unwrap(),
                auth_url: get_var("SELECTEL_AUTH_URL"),
                zone_url: get_var("SELECTEL_ZONE_URL"),
            },
        }
    }
}