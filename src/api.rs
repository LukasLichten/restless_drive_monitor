use actix_web::{web::{Json, Data, Path}, get};
use reqwest::Client;

use crate::{smart, Config, data::{Blockdevice, Alert, AlertLevel}, truenas};


#[get("/ping")]
pub async fn get_ping() -> Json<String> {
    Json("pong".to_string())
}

#[get("/drivelist")]
pub async fn get_drive_list() -> Option<Json<Vec<Blockdevice>>> {
    Some(Json(smart::get_disks()?))
}

#[get("/alerts")]
pub async fn get_alerts(config: Data<Config>, client: Data<Client>) -> Option<Json<Vec<Alert>>> {
    if !config.use_truenas {
        return None;
    }

    let data = truenas::get_alerts(&client, &(config.truenas_address.clone()?), &config.truenas_token).await?;
    
    Some(Json(data))
}

#[get("/alerts/{level}")]
pub async fn get_alerts_on_level(config: Data<Config>, client: Data<Client>, level: Path<String>) -> Option<Json<Vec<Alert>>> {
    if !config.use_truenas {
        return None;
    }

    let minimum = match level.to_lowercase().as_str() {
        "info" => AlertLevel::Info,
        "warning" => AlertLevel::Warning,
        "critical" => AlertLevel::Critical,
        _ => return None
    };

    let data = truenas::get_alerts(&client, &(config.truenas_address.clone()?), &config.truenas_token).await?;
    let filtered_data = data.into_iter().filter(|item| !item.dismissed && item.level >= minimum).collect();
    
    Some(Json(filtered_data))
}

#[get("/test")]
pub async fn get_test(config: Data<Config>, client: Data<Client>) -> Option<Json<String>> {
    if !config.use_truenas {
        return None;
    }

    let data = truenas::request(&client, &(config.truenas_address.clone()?), &config.truenas_token, "core/ping".to_string()).await?;
    
    Some(Json(String::from_utf8(data).ok()?))
}