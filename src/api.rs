use log::{debug, error, info};
use poem::IntoResponse;
use poem_openapi::{
    param::{Path, Query},
    payload::{Json, Payload},
    ApiResponse, OpenApi,
};
use reqwest::Client;

use crate::{
    data::{Alert, AlertLevel, ApiServices, Blockdevice, Smart},
    smart, truenas, Config,
};

pub struct Api {
    config: Config,
    smart_enabled: bool,
    truenas_enabled: bool,
    client: Client,
}

pub fn new_api(config: Config) -> Api {
    fn is_truenas(config: &Config) -> (bool, Client) {
        if config.use_truenas {
            if let (Some(token), Some(address)) = (&config.truenas_token, &config.truenas_address) {
                if !token.is_empty() && !address.cannot_be_a_base() {
                    if let Some(client) = truenas::get_client(config.accept_invalid_certs) {
                        info!("TrueNAS support enabled!");
                        return (true, client);
                    }
                    error!("Failed to create Web Client, no TrueNAS support");
                    return (false, reqwest::Client::new());
                }
            }
            error!("No Address and or Token provided, no TrueNAS support")
        }

        (false, reqwest::Client::new())
    }

    let (truenas_enabled, client) = is_truenas(&config);

    let smart_enabled = if cfg!(target_os = "linux") {
        if nix::unistd::Uid::effective().is_root() {
            info!("Smart support enabled");
            true
        } else {
            false
        }
    } else {
        false
    };

    Api {
        config,
        truenas_enabled,
        smart_enabled,
        client,
    }
}

#[derive(ApiResponse)]
pub enum RdmResponde<T>
where
    T: Payload,
    T: IntoResponse,
{
    /// Okay
    #[oai(status = 200)]
    Ok(T),
    /// Item you requested could not be found
    #[oai(status = 410)]
    NotFound,
    /// An internal error occurred
    #[oai(status = 500)]
    InternalServerError,
    /// This methode is part of a service that is disabled, please check the server config and /services to enable this methode
    #[oai(status = 503)]
    ServiceDisabled,
}

#[OpenApi]
impl Api {
    /// Ping the pong with the api
    #[oai(path = "/ping", method = "get")]
    pub async fn get_ping(&self) -> Json<String> {
        debug!("pong");
        Json("pong".to_string())
    }

    /// Returns the Services available with on this server
    ///
    /// This function serves to debug the configuartion, as certain functions require certain services to work.<br>
    /// The state of the "_enabled" values won't change till a restart of the server, but the `truenas_status` is however an active connection test.<br>  
    /// Although false values there could also indicate improper configuartion (like incorrect token or server address).<br>
    /// And if `truenas_enabled` is false it will always be false.<br>
    /// `smart_enabled` is false when you are runnning without root (or not on a linux system)
    #[oai(path = "/services", method = "get")]
    pub async fn get_services(&self) -> Json<ApiServices> {
        Json(ApiServices {
            truenas_enabled: self.truenas_enabled,
            smart_enabled: self.smart_enabled,
            truenas_status: if self.truenas_enabled {
                truenas::do_ping(
                    &self.client,
                    self.config
                        .truenas_address
                        .as_ref()
                        .expect("When truenas is enabled the address has to be set"),
                    self.config
                        .truenas_token
                        .as_ref()
                        .expect("When truenas is enabled the token has to be set"),
                )
                .await
                .unwrap_or(false)
            } else {
                false
            },
        })
    }

    /// Returns all the disks
    #[oai(path = "/drivelist", method = "get")]
    pub async fn get_drive_list(&self) -> RdmResponde<Json<Vec<Blockdevice>>> {
        if let Some(disks) = smart::get_disks() {
            return RdmResponde::Ok(Json(disks));
        }

        RdmResponde::InternalServerError
    }

    /// Returns Smart Data for a certain drive via simple name
    /// 
    /// This function requires smart_enabled, check `/services`  
    ///
    /// * `drive` - name of the drive, for example "sda"
    #[oai(path = "/smart/:drive", method = "get")]
    pub async fn get_smart_data(&self, drive: Path<String>) -> RdmResponde<Json<Smart>> {
        if let Some(disks) = smart::get_disks() {
            // Sanetize input
            for item in disks {
                if item.name == drive.clone() {
                    return self.smart_reader(item.name);
                }
            }

            return RdmResponde::NotFound;
        }

        RdmResponde::InternalServerError
    }

    /// Returns Smart Data for a certain drive based on disk-id
    /// 
    /// This function requires smart_enabled, check `/services`
    ///
    /// * `drive` - disk-id of the drive (defined by /dev/disk/by-id/ on the machine), which you can retrieve via [`/drivelist`](crate::api::Api::get_drive_list)
    #[oai(path = "/smart/disk/by-id/:drive", method = "get")]
    pub async fn get_smart_data_by_id(&self, drive: Path<String>) -> RdmResponde<Json<Smart>> {
        if let Some(disks) = smart::get_drive_id_list() {
            // Sanetize input
            for (id, _target) in disks {
                if id == drive.clone() {
                    return self.smart_reader(format!("disk/by-id/{}", id).to_string());
                }
            }

            return RdmResponde::NotFound;
        }

        RdmResponde::InternalServerError
    }

    fn smart_reader(&self, drive: String) -> RdmResponde<Json<Smart>> {
        if !self.smart_enabled {
            return RdmResponde::ServiceDisabled;
        }

        if let Some(info) = smart::get_smart(drive) {
            return RdmResponde::Ok(Json(info));
        }

        RdmResponde::InternalServerError
    }

    /// Returns all current alerts
    /// 
    /// This function requires truenas_enabled, check `/services`
    /// 
    /// * `level` - minimum alert level, can be either Info, Warning, Critical
    /// * `include_dismissed` - include also dismissed alerts (per default they are ignored)
    #[oai(path = "/alerts", method = "get")]
    pub async fn get_alerts_on_level(&self, level: Query<Option<String>>, include_dismissed: Query<Option<bool>>) -> RdmResponde<Json<Vec<Alert>>> {
        if !self.truenas_enabled {
            return RdmResponde::ServiceDisabled;
        }

        let level = level.0.unwrap_or("warning".to_string());
        let include_dismissed = include_dismissed.0.unwrap_or(false);

        let minimum = match level.to_lowercase().as_str() {
            "info" => AlertLevel::Info,
            "warning" => AlertLevel::Warning,
            "critical" => AlertLevel::Critical,
            _ => return RdmResponde::NotFound,
        };

        let res = truenas::get_alerts(
            &self.client,
            self.config.truenas_address.as_ref().expect("When truenas is enabled the address has to be set"),
            self.config.truenas_token.as_ref().expect("When truenas is enabled the address has to be set"),
        )
        .await;

        if let Some(data) = res {
            let filtered_data: Vec<Alert> = data
                .into_iter()
                .filter(|item| (!item.dismissed || include_dismissed) && item.level >= minimum)
                .collect();
            
            return RdmResponde::Ok(Json(filtered_data));
        }

        RdmResponde::InternalServerError
    }
}
