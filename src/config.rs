use toml;
use serde::Deserialize;
use std::fs;

/// GLOBAL_CONFIG = $HOME/.config/trans-cli-rs/config.toml
pub static mut GLOBAL_CONFIG: String = String::new();

pub trait ParseConfig where Self: Sized {
    fn parse_config(config_path: String) -> Result<Self, std::io::Error>;
}

#[derive(Default, Debug, Deserialize, PartialEq, Eq)]
pub struct Key {
    pub(crate) appid: String,
    pub(crate) secert_key: String,
}

#[derive(Default, Debug, Deserialize, PartialEq, Eq)]
pub struct Basic {
    pub(crate) backend: String,
    pub(crate) from: String,
    pub(crate) to: String,
}

#[derive(Default, Debug, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub(crate) basic: Basic,
    pub(crate) key: Key,
}
