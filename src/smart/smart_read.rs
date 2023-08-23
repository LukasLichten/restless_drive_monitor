use std::process::Command;

use serde::Deserialize;

use crate::data;

pub fn get_smart(drive: String) -> Option<data::Smart> {
    if cfg!(target_os = "windows") {
        return None;
    }

    let output = Command::new("smartctl")
        .arg("-j")
        .arg("-H")
        .arg("-A")
        .arg(format!("/dev/{}", drive))
        .output().ok()?;

    if let Ok(res) = serde_json::from_slice(output.stdout.as_slice()) {
        let res: SmartResult = res;

        return Some(res.parse());
    }

    None
}

#[derive(Debug, Clone, Deserialize)]
struct SmartResult {
    smart_status: SmartStatus,
    ata_smart_attributes: AttributeContainer,
    power_cycle_count: u64,
    power_on_time: PowerTime,
    device: data::SmartDevice
}

#[derive(Debug, Clone, Deserialize)]
struct SmartStatus {
    passed: bool
}

#[derive(Debug, Clone, Deserialize)]
struct AttributeContainer {
    table: Vec<SmartAttribute>
}

#[derive(Debug, Clone, Deserialize)]
struct SmartAttribute {
    id: u16,
    name: String,
    value: u8,
    worst: u8,
    thresh: u8,
    raw: SmartRaw,
    flags: data::SmartFlags
}

#[derive(Debug, Clone, Deserialize)]
struct SmartRaw {
    value: u64
}

#[derive(Debug, Clone, Deserialize)]
struct PowerTime {
    hours: u64
}

impl SmartResult {
    fn parse(self) -> data::Smart {

        let attributes = self.ata_smart_attributes.table.into_iter().map(|item| {
            item.parse()
        }).collect();

        let mut caution = false;
        for item in &attributes {
            let item: &data::SmartAttribute = item;
            if item.caution {
                caution = true;
                break;
            }
        }

        data::Smart {
            passed: self.smart_status.passed,
            device: self.device,
            power_on_hours: self.power_on_time.hours,
            power_cycle_count: self.power_cycle_count,
            attributes,
            caution
        }
    }
}

impl SmartAttribute {
    fn parse(self) -> data::SmartAttribute {
        let raw = self.raw.value;

        let caution = if self.worst <= self.thresh {
            true
        } else {
            match self.id {
                0x01 => { // Read Error Rate
                    // Older drives may just have an increase of these without signifying real errors, so for now we ignore
                    false
                },
                0x05 => { // Reallocated Sectors Count
                    raw > 0 // Anything larger then zero indicates imminent failure
                },
                0x0A => { // Spin Retry Count
                    raw > 1
                },
                0xC4 => { // Reallocation Event Count
                    raw > 0
                },
                0xC5 => { // Current Pending Sector Count
                    raw > 0
                },
                0xC6 => { // Uncorrectable Sector Count
                    raw > 0
                },

                _ => false,
            }
        };
        
        data::SmartAttribute {
            id: self.id,
            name: self.name,
            value: self.value,
            worst: self.worst,
            threshold: self.thresh,
            raw,
            flags: self.flags,
            caution
        }
    }
}