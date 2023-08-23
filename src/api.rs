use actix_web::{web::{Json, Data, Path}, get};
use reqwest::Client;

use crate::{smart, Config, data::{Blockdevice, Alert, AlertLevel, Smart}, truenas};


#[get("/ping")]
pub async fn get_ping() -> Json<String> {
    Json("pong".to_string())
}

#[get("/drivelist")]
pub async fn get_drive_list() -> Option<Json<Vec<Blockdevice>>> {
    Some(Json(smart::get_disks()?))
}

#[get("/smart/{drive}")]
pub async fn get_smart_data(drive: Path<String>) -> Option<Json<Smart>> {
    // Sanetize input
    for item in smart::get_blockdevices()? {
        if item.name == drive.clone() {
            return smart_reader(drive.clone());
        }
    }

    None
}

#[get("/smart/disk/by-id/{drive}")]
pub async fn get_smart_data_by_id(drive: Path<String>) -> Option<Json<Smart>> {
    // Sanetize input
    for (id, _target) in smart::get_drive_id_list()? {
        if id == drive.clone() {
            return smart_reader(format!("disk/by-id/{}", drive).to_string());
        }
    }

    None
}

fn smart_reader(drive: String) -> Option<Json<Smart>> {
    return Some(Json(smart::get_smart(drive)?));
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
