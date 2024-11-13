use tracing::log::{info, warn};

pub struct RheiConfig {
    pub ip: String,
    pub port: u16,
}

impl Default for RheiConfig {
    fn default() -> Self {
        RheiConfig {
            ip: String::from("0.0.0.0"),
            port: 3000,
        }
    }
}

pub fn load_config() -> RheiConfig {
    use dotenvy::dotenv;
    use std::env;

    let mut config = RheiConfig::default();

    match dotenv() {
        Ok(_) => info!(".env file found, loading config from it."),
        Err(_) => info!(".env file not found."),
    }

    let ip = env::var("RHEI_IP").ok();
    let port = env::var("RHEI_PORT").ok();

    match ip {
        Some(ip) => config.ip = ip,
        None => warn!("missing 'RHEI_IP' environment variable, using default."),
    }
    match port {
        Some(port) => {
            let port = port.parse::<u16>();
            match port {
                Ok(port) => config.port = port,
                Err(_) => {
                    warn!("found 'RHEI_PORT' environment variable but it must be a u16, using default.")
                }
            }
        }
        None => warn!("missing 'RHEI_PORT' environment variable, using default."),
    }

    config
}
