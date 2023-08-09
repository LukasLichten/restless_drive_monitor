use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::data;

pub fn get_client(accept_invalid_certs: bool) -> Option<Client> {
    Some(reqwest::Client::builder()
        .danger_accept_invalid_certs(accept_invalid_certs)
        .build()
        .ok()?)
}


pub async fn request(client: &Client, address: &Url, token: &String, api_function: String) -> Option<Vec<u8>> {
    let target = address.join("/api/v2.0/").ok()?.join(&api_function).ok()?;
    let req = client.get(target).bearer_auth(token).build().ok()?;
    let res = client.execute(req).await.ok()?;
    let result = res.bytes().await.ok()?;
    Some(result.into())
}

pub async fn get_alerts(client: &Client, address: &Url, token: &String) -> Option<Vec<data::Alert>> {
    let internal: Vec<InternalAlert> = serde_json::from_slice(request(client, address, token, "alert/list".to_string()).await?.as_slice()).ok()?;

    Some(internal.into_iter().map(|item| {
        item.parse()
    }).collect())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InternalAlert {
    uuid: Uuid,
    source: String,
    klass: String,
    node: String,
    dismissed: bool,
    #[serde(rename(deserialize = "formatted"))]
    text: String,
    level: String,
    one_shot: bool,
    datetime: Time,
    last_occurrence: Time
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Time {
    #[serde(rename(deserialize = "$date"))]
    date: u64
}

impl InternalAlert {
    fn parse(self) -> data::Alert {
        data::Alert {
            uuid: self.uuid,
            source: self.source,
            klass: self.klass,
            node: self.node,
            dismissed: self.dismissed,
            text: self.text,
            level: match self.level.as_str() {
                "INFO" => data::AlertLevel::Info,
                "WARNING" => data::AlertLevel::Warning,
                "CRITICAL" => data::AlertLevel::Critical,
                _ => data::AlertLevel::Unknown
            },
            one_shot: self.one_shot,
            datetime_ms: self.datetime.date,
            last_occurrence_ms: self.last_occurrence.date,
        }
    }
}
