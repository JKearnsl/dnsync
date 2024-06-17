use log::info;
use serde_json::json;

pub async fn make_token(
    client: &reqwest::Client,
    username: &str,
    account_id: u32,
    password: &str,
    project: &str,
    auth_url: &str
) -> String {

    info!("Making token for user: {}", username);

    let payload = json!({
        "auth": {
            "identity": {
                "methods": ["password"],
                "password": {
                    "user": {
                        "name": username,
                        "domain": {
                            "name": account_id.to_string()
                        },
                        "password": password
                    }
                }
            },
            "scope": {
                "project": {
                    "name": project,
                    "domain": {
                        "name": account_id.to_string()
                    }
                }
            }
        }
    });

    match client.post(auth_url).json(&payload).send().await {
        Ok(response) => match response.headers().get("x-subject-token") {
            Some(token) => token.to_str().unwrap().to_string(),
            None => {
                log::error!("Token not found");
                std::process::exit(1);
            }
        },
        Err(error) => {
            log::error!("Failed to get token: {}", error);
            std::process::exit(1);
        }
    }
}
