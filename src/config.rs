use toml;
use serde::{Deserialize, Serialize};
use std::fs;

pub trait ParseConfig where Self: Sized {
    fn parse_config(config_path: String) -> Result<Self, std::io::Error>;
}

#[derive(Default, Debug, Deserialize, PartialEq, Eq)]
pub struct BasicConfig {
    pub(crate) appid: String,
    pub(crate) secert_key: String,
}
