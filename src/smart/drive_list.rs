use std::process::Command;

use serde::{Deserialize, Serialize};

// Using lsblk we get a list of drives
// We need this because we need the drive letters for smartctl

#[derive(Serialize, Deserialize)]
struct Blocklist {
    blockdevices: Vec<Blockdevice>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Blockdevice {
    pub name: String,
    #[serde(rename(deserialize = "rm"))]
    pub removable: bool,
    pub size: String,
    #[serde(rename(deserialize = "ro"))]
    pub read_only: bool,
    pub mountpoint: Option<String>,
    #[serde(rename = "type")]
    pub drive_type: String,
    #[serde(rename = "maj:min")]
    pub maj_min: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Blockdevice>>
}

pub fn get_blockdevices() -> Option<Vec<Blockdevice>>{
    if cfg!(target_os = "windows") {
        return None;
    }

    let output = Command::new("lsblk")
        .arg("-J")
        .output().ok()?;

    if let Ok(res) = serde_json::from_slice(output.stdout.as_slice()) {
        let res: Blocklist = res;
        return Some(res.blockdevices);
    }

    None
}

pub fn get_disks() -> Option<Vec<Blockdevice>> {
    Some(get_blockdevices()?
            .into_iter()
            .filter(|device| device.drive_type.as_str() == "disk" )
            .collect())
}