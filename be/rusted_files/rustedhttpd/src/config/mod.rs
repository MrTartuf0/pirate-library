use serde::Serialize;
use serde::Deserialize;

use serde_yaml;

use std::fs::File;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub server: ServerSettings,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerSettings {
    #[serde(default = "default_port")]
    pub port: u32,
    #[serde(default = "default_threads")]
    pub threads: usize,
    #[serde(default = "default_ttl")]
    pub ttl: u32,
    #[serde(default = "default_root")]
    pub root: String,
}

fn default_port() -> u32 {
    println!("Unspecified port, using default: 8453");
    8453
}

fn default_threads() -> usize {
    println!("Unspecified threads, using default: 2");
    2
}

fn default_ttl() -> u32 {
    println!("Unspecified ttl, using default: 10");
    10
}

fn default_root() -> String {
    println!("Unspecified root, using default: ./web");
    "./web".to_string()
}

pub fn parse_config() -> ServerConfig {
    let file = File::open("./config.yaml").unwrap();
    let config: ServerConfig = serde_yaml::from_reader(file).unwrap();
    config
}
