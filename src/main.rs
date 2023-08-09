mod api;
pub mod smart;
pub mod truenas;
pub mod data;

use std::fs;

use actix_web::{HttpServer, App, middleware::Logger, web::Data};
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    
    if let Some(config) = get_config() {

        HttpServer::new(move || {
            let logger = Logger::default();
            let config = get_config().expect("We could load the config once, we can load it another time");
    
            let mut app = App::new()
            .wrap(logger)
            .service(api::get_ping)
            .service(api::get_drive_list);
    
            if config.use_truenas {
                if let Some(client) = truenas::get_client(config.accept_invalid_certs) {
                    app = app
                        .app_data(Data::new(config))
                        .app_data(Data::new(client))
                        .service(api::get_alerts)
                        .service(api::get_alerts_on_level)
                }
            }
    
            app
        })
        .bind(("0.0.0.0", config.port))?
        .run()
        .await
    } else {
        println!("Failed to parse config file, aborting launch");
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub use_truenas: bool,
    pub truenas_address: Option<url::Url>,
    pub truenas_token: String,
    pub accept_invalid_certs: bool,
    pub port: u16
}

pub fn get_config() -> Option<Config> {
    let path = std::path::PathBuf::from("./rdm.conf");
    if !path.exists() {
        fs::write(&path, serde_json::to_string_pretty(&Config {
            use_truenas: false, truenas_address: Some(url::Url::parse("http://localhost/").ok()?), truenas_token: "".to_string(),
            accept_invalid_certs: false, port: 30603
        }).ok()?.as_bytes()).ok()?;
    } else if path.is_dir() {
        return None;
    }

    Some(serde_json::from_slice(fs::read(&path).ok()?.as_slice()).ok()?)
}