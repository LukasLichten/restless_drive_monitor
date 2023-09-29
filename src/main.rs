mod api;
pub mod smart;
pub mod truenas;
pub mod data;
mod installer;

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

use std::fs;

//use actix_web::{HttpServer, App, middleware::Logger, web::Data};
use clap::Parser;
use log::{error, info};
use poem::{Route, listener::TcpListener};
use poem_openapi::OpenApiService;
use serde::{Deserialize, Serialize};

//use crate::api::Api;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let log_level = if built_info::DEBUG { log::LevelFilter::Debug } else { log::LevelFilter::Info };
    env_logger::builder().filter_level(log_level).init();
    

    if args.install {
        let res = installer::install();
        if let Err(e) = res {
            error!("Failed to install properly");
            return Err(e);
        }

        return Ok(()); 
    }
    
    if let Some(config) = get_config() {
        let port = config.port;
        info!("Launching Server on port {}", port);

        let api_service = OpenApiService::new(api::new_api(config), "Restless Drive Monitor", built_info::PKG_VERSION).server("/v1.0");
        
        let doc = api_service.swagger_ui();
        let app = Route::new()
                .nest("/v1.0", api_service)
                .nest("/doc", doc);

        poem::Server::new(TcpListener::bind(format!("0.0.0.0:{}", port)))
            .run(app)
            .await
    } else {
        error!("Failed to parse config file, aborting launch");
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub use_truenas: bool,
    pub truenas_address: Option<url::Url>,
    pub truenas_token: Option<String>,
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
        let file = fs::File::create(&path).ok()?;

        if cfg!(target_os = "linux") {
            use std::os::unix::fs::PermissionsExt;

            let mut perm = file.metadata().ok()?.permissions();
            perm.set_mode(0o600);
            file.set_permissions(perm).ok()?;
        }
        

        fs::write(&path, serde_json::to_string_pretty(&Config {
            use_truenas: false, truenas_address: Some(url::Url::parse("http://localhost/").ok()?), truenas_token: Some("".to_string()),
            accept_invalid_certs: false, port: 30603
        }).ok()?.as_bytes()).ok()?;
    } else if path.is_dir() {
        return None;
    }

    if cfg!(target_os = "linux") {
        use std::os::unix::fs::PermissionsExt;

        let perm = fs::File::open(&path).ok()?.metadata().ok()?.permissions();
        if perm.mode() != 0o600 && perm.mode() != 0o200 {
            error!("Permission on config file are too broad, this presents a security risk, use 600 or 200");
        }
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