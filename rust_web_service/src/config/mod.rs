use log::{debug, info, trace, warn};
use serde_derive::Deserialize;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct ServerConfig {
    pub address: SocketAddr,
}

pub fn get_config() -> Option<ServerConfig> {
    let config = File::open("microservice.toml")
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

    //let config = File::open("microservice.toml");

    config
}
