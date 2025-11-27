// src/config.rs
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub seed_node: bool,
    pub node_id: String,
    pub public_ip: String,
    pub port: u16,
    pub high_neuron_mode: bool,
    pub listen_addr: Option<String>,
    pub enable_mdns: Option<bool>,
}

impl Config {
    pub fn load() -> Self {
        let config_str = fs::read_to_string("config.toml").unwrap_or_default();
        if config_str.is_empty() {
            return Self::default();
        }
        toml::from_str(&config_str).unwrap_or_else(|_| Self::default())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            seed_node: false,
            node_id: "pappap-node-001".to_string(),
            public_ip: "0.0.0.0".to_string(),
            port: 36331,
            high_neuron_mode: false,
            listen_addr: Some("0.0.0.0:36331".to_string()),
            enable_mdns: Some(true),
        }
    }
}