use std::process::Command;

use serde::Deserialize;

// Using lsblk we get a list of drives
// We need this because we need the drive letters for smartctl

#[derive(Deserialize)]
struct Blocklist {
    blockdevices: Vec<Blockdevice>
}

#[derive(Deserialize, Clone)]
struct Blockdevice {
    name: String,
    #[serde(rename(deserialize = "rm"))]
    removable: bool,
    size: String,
    #[serde(rename(deserialize = "ro"))]
    read_only: bool,
    mountpoint: Option<String>,
    #[serde(rename = "type")]
    drive_type: String,
    #[serde(rename = "maj:min")]
    maj_min: String,

    children: Option<Vec<Blockdevice>>
}

pub fn get_blockdevices() -> Option<Vec<crate::data::Blockdevice>>{
    if cfg!(target_os = "windows") {
        return None; // There is a library to retrieve windows data, so if there is a usecase: TODO
    }

    let output = Command::new("lsblk")
        .arg("-J")
        .output().ok()?;

    if let Ok(res) = serde_json::from_slice(output.stdout.as_slice()) {
        let res: Blocklist = res;
        return Some(res.parse());
    }

    None
}

pub fn get_disks() -> Option<Vec<crate::data::Blockdevice>> {
    Some(get_blockdevices()?
            .into_iter()
            .filter(|device| device.device_type.as_str() == "disk" )
            .collect())
}

impl Blocklist {
    fn parse(self) -> Vec<crate::data::Blockdevice> {
        self.blockdevices.into_iter().map(|item| {
            item.parse()
        }).collect()
    }
}

impl Blockdevice {
    fn parse(self) -> crate::data::Blockdevice {
        let mut text = self.size.replace(",", ".").to_lowercase();
        
        let size = if let Some(size_letter) = text.pop() {
            let temp = if let Ok(try_size) = text.parse::<f64>() {
                try_size
            } else {
                0.0
            };


            let mut size = match size_letter {
                'k' => temp,
                'm' => temp * 1024.0,
                'g' => temp * 1024_u64.pow(2) as f64,
                't' => temp * 1024_u64.pow(3) as f64,
                'p' => temp * 1024_u64.pow(4) as f64,
                _ => 0.0
            };
            size = size * 1024.0 / 1000.0;

            size as u64
        } else {
            0
        };
        

        let childs = if let Some(children) = self.children {
            Some(children.into_iter().map(|item| {
                item.parse()
            }).collect())
        } else {
            None
        };

        crate::data::Blockdevice {
            name: self.name,
            removable: self.removable,
            size_kb: size,
            read_only: self.read_only,
            mountpoint: self.mountpoint,
            device_type: self.drive_type,
            maj_min: self.maj_min,
            children: childs
        }
    }
}

