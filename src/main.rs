mod api;
pub mod smart;
pub mod truenas;
pub mod data;
mod installer;

use std::fs;

use actix_web::{HttpServer, App, middleware::Logger, web::Data};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.install {
        return installer::install();
    }

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    
    if let Some(config) = get_config() {
        println!("Launching Server on port {}", config.port);

        HttpServer::new(move || {
            let logger = Logger::default();
            let config = get_config().expect("We could load the config once, we can load it another time");
            
            let mut app = App::new()
            .wrap(logger)
            .service(api::get_ping)
            .service(api::get_drive_list);

            if cfg!(target_os = "linux") {
                if nix::unistd::Uid::effective().is_root() {
                    println!("Smart support enabled"); // For some reason, this and the TrueNAS message, get printed twice

                    app = app
                    .service(api::get_smart_data)
                    .service(api::get_smart_data_by_id);
                }
            }
    
            if config.use_truenas {
                if let Some(client) = truenas::get_client(config.accept_invalid_certs) {
                    println!("TrueNAS support enabled");

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
    let args = Args::parse();

    let path = std::path::PathBuf::from(match args.config {
        Some(p) => p,
        None => "./rdm.conf".to_string()
    });
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

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Installs the software in /usr/bin and creates a service to run it")]
    install: bool,

    #[arg(short, long, help = "Where the config file is located")]
    config: Option<String>
} 