use serde::{Serialize, Deserialize};
use uuid::Uuid;

// These are used to communicate on the api
// Intended to be able to copy into your application to easily talk to this backend

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blockdevice {
    pub name: String,
    pub removable: bool,
    pub size_kb: u64,
    pub read_only: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mountpoint: Option<String>,
    #[serde(rename = "type")]
    pub device_type: String,
    #[serde(rename = "maj:min")]
    pub maj_min: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Blockdevice>>
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Alert {
    pub uuid: Uuid,
    pub source: String,
    pub klass: String,
    pub node: String,
    pub dismissed: bool,
    #[serde(rename(deserialize = "formatted"))]
    pub text: String,
    pub level: AlertLevel,
    pub one_shot: bool,
    pub datetime_ms: u64,
    pub last_occurrence_ms: u64
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
    Unknown
}