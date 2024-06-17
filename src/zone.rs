use log::info;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct ZoneItem {
    id: String,
    name: String,
}

#[derive(Deserialize)]
struct ZoneResult {
    result: Vec<ZoneItem>,
}


pub async fn get_zone_by_name(
    client: &reqwest::Client,
    zone_name: &str,
    zone_url: &str,
    token: &str
) -> String {

    info!("Getting zone by name: {}", zone_name);

    match client.get(format!("{zone_url}/zones")).header(
        "X-Auth-Token", token
    ).query(&[("filter", zone_name)]).send().await {
        Ok(response) => match response.json::<ZoneResult>().await {
            Ok(zone) => {

                let zone = zone.result.iter()
                    .find(|zone| zone.name == zone_name);

                if zone.is_none() {
                    log::error!("Zone not found by name {}", zone_name);
                    std::process::exit(1);
                }

                zone.unwrap().id.to_string()
            },
            Err(error) => {
                log::error!("Failed to parse json: {}", error);
                std::process::exit(1);
            }
        },
        Err(error) => {
            log::error!("Failed to get zone: {}", error);
            std::process::exit(1);
        }
    }
}


#[derive(Deserialize, Debug)]
struct RecordItem {
    content: String,
}


#[derive(Deserialize, Debug)]
struct RecordResultItem {
    id: String,
    name: String,
    #[serde(rename = "type")]
    _type: String,

    records: Vec<RecordItem>,
}

#[derive(Deserialize)]
struct RecordResult {
    result: Vec<RecordResultItem>,
}

pub async fn get_record_by_name(
    client: &reqwest::Client,
    name: &str,
    zone_id: &str,
    zone_url: &str,
    token: &str,
) -> Option<(String, String)> {
    match client.get(format!("{zone_url}/zones/{zone_id}/rrset")).header(
        "X-Auth-Token", token
    ).query(&[("search", name)]).send().await {
        Ok(response) => match response.json::<RecordResult>().await {
            Ok(record) => {
                match record.result.iter()
                    .find(|record| record.name == name && record._type == "A") {
                    Some(record_result) => {
                        let record_item = record_result.records.first().unwrap();
                        let record_ip = record_item.content.to_string();
                        let record_id = record_result.id.to_string();
                        Some((record_id, record_ip))
                    },
                    None => None
                }
            },
            Err(error) => {
                log::error!("Failed to parse json: {}", error);
                std::process::exit(1);
            }
        },
        Err(error) => {
            log::error!("Failed to get record: {}", error);
            std::process::exit(1);
        }
    }
}
pub async fn add_record(
    client: &reqwest::Client,
    zone_id: &str,
    name: &str,
    ipv4: &str,
    record_type: &str,
    ttl: u32,
    zone_url: &str,
    token: &str,
) {
    let payload = json!({
        "name": name,
        "ttl": ttl,
        "type": record_type,
        "records": [
            {
              "content": ipv4,
              "disabled": false
            }
        ],
        "comment": "By dnsync"
    });

    match client.post(format!("{zone_url}/zones/{zone_id}/rrset")).header(
        "X-Auth-Token", token
    ).json(&payload).send().await {
        Ok(response) => {
            if response.status().is_success() {
                info!("Record added: {}", name);
            } else {
                log::error!("Failed to add record: {}", response.status());
                std::process::exit(1);
            }
        },
        Err(error) => {
            log::error!("Failed to add record: {}", error);
            std::process::exit(1);
        }
    }
}

pub async fn update_record_content(
    client: &reqwest::Client,
    zone_id: &str,
    record_id: &str,
    ipv4: &str,
    ttl: u32,
    comment: &str,
    zone_url: &str,
    token: &str,
) {
    let payload = json!({
        "ttl": ttl,
        "records": [
            {
              "content": ipv4,
              "disabled": false
            }
        ],
        "comment": comment
    });

    match client.patch(format!("{zone_url}/zones/{zone_id}/rrset/{record_id}")).header(
        "X-Auth-Token", token
    ).json(&payload).send().await {
        Ok(response) => {
            if response.status().is_success() {
                info!("Record updated: {}", record_id);
            } else {
                log::error!("Failed to update record: {}", response.status());
                std::process::exit(1);
            }
        },
        Err(error) => {
            log::error!("Failed to update record: {}", error);
            std::process::exit(1);
        }
    }
}
