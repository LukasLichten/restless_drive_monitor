use poem_openapi::{Object, Enum};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

// These are used to communicate on the api
// Intended to be able to copy into your application to easily talk to this backend

#[derive(Debug, Serialize, Deserialize, Object, Clone)]
pub struct ApiServices {
    /// Set on bootup, is true if use_truenas is true and truenas_address and truenas_token are set
    pub truenas_enabled: bool,
    /// Set on bootup, is true when user has root access (and is on Linux)
    pub smart_enabled: bool,
    /// False if truenas_enabled is false, else this is the result of a test querry to the truenas api
    pub truenas_status: bool
}

/// A Blockdevice conntected to the machine, this can be a physical, partion, or virtual drive
#[derive(Debug, Serialize, Deserialize, Object, Clone)]
#[oai(skip_serializing_if_is_none)]
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
    #[oai(rename = "maj:min")]
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
    /// List of child blockdevices, usually partitions
    pub children: Option<Vec<Blockdevice>>
}

/// A Alert/Notification from TrueNAS
#[derive(Debug, Serialize, Deserialize, Clone, Object)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Enum)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
    Unknown
}

/// Smart Data from a Drive
#[derive(Debug, Serialize, Deserialize,Clone, Object)]
pub struct Smart {
    pub device: SmartDevice,
    pub passed: bool,
    /// This number is based on the Power_On_Hours Attribute raw value, which from personal expierence, is potentially not 1:1 hours
    pub power_on_hours: u64,
    pub power_cycle_count: u64,
    pub attributes: Vec<SmartAttribute>,
    /// Evaluated by this programm, as vendors are often way too lax on certain values
    /// This is a summary of all attributes, and returns true if any are on caution
    pub caution: bool 
}

/// General Device information
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct SmartDevice {
    pub name: String,
    #[serde(rename = "type")]
    #[oai(rename = "type")]
    pub device_type: String,
    pub protocol: String
}

/// A specific Smart Attribute
#[derive(Debug, Serialize, Deserialize,Clone, Object)]
pub struct SmartAttribute {
    pub id: u16,
    pub name: String,
    /// Normalized value between 1-253, usually higher is better, usually starts at 100, but vendors can do whatever they want to
    pub value: u8,
    /// Worst value ever observed (usually the same as value)
    pub worst: u8,
    /// Threashold for the Normalized value for this attribute to be marked as failed
    pub threshold: u8,
    /// Vendor specific 8 byte block, but regularly is a counter
    pub raw: u64,
    pub flags: SmartFlags,
    /// Evaluated by this programm, as vendors are often way too lax on certain values
    /// Will evaluate for certain high risk attributes the raw value, else it will flag fail if the worst drops below threashold
    pub caution: bool,
}

/// Flags of a Smart Attribute
#[derive(Debug, Clone, Deserialize, Serialize, Object)]
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