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
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none", rename = "wwn")]
    pub world_wide_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,

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

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Smart {
    pub device: SmartDevice,
    pub passed: bool,
    pub power_on_hours: u64,
    pub power_cycle_count: u64,
    pub attributes: Vec<SmartAttribute>,
    pub caution: bool // Evaluated by this programm, as vendors are often way too lax on certain values
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartDevice {
    pub name: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub protocol: String
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct SmartAttribute {
    pub id: u16,
    pub name: String,
    pub value: u8,
    pub worst: u8,
    pub threshold: u8,
    pub raw: u64,
    pub flags: SmartFlags,
    pub caution: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SmartFlags {
    pub value: u8,
    pub string: String,
    pub prefailure: bool,
    pub updated_online: bool,
    pub performance: bool,
    pub error_rate: bool,
    pub event_count: bool,
    pub auto_keep: bool
}