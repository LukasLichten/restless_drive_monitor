use actix_web::{web::Json, get};

use crate::smart;


#[get("/ping")]
pub async fn get_ping() -> Json<String> {
    Json("pong".to_string())
}

#[get("/drivelist")]
pub async fn get_drive_list() -> Option<Json<Vec<smart::Blockdevice>>> {
    Some(Json(smart::get_disks()?))
}