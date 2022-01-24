use log::{debug, info, trace, warn};
use serde_derive::Deserialize;
use std::fs::File;
use std::io::{self, Read};
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub address: SocketAddr,
}

pub fn get_config() -> Option<ServerConfig> {
    let config = File::open("settings.toml")
        .and_then(|mut file| {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            Ok(buffer)
        })
        .and_then(|buffer| {
            toml::from_str::<ServerConfig>(&buffer)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
        })
        .map_err(|err| {
            warn!("Can't read config file: {}", err);
        })
        .ok();

    config
}
