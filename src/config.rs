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

impl Config {
    pub fn load(path: String) -> Result<Self, std::io::Error> {
        let data = fs::read(path)?;
        let result: Config = toml::from_slice(data.as_ref())?;

        Ok(result)
    }
}

#[test]
fn test_load_config() {
    let path = "fixtures/test_config.toml".to_owned();
    let config = Config::load(path).unwrap();
    assert_eq!(Config {
        basic: Basic { backend: "baidu_trans".to_owned(), from: "en".to_owned(), to: "zh".to_owned() },
        key: Key { appid: "appid_123".to_owned(), secert_key: "key_123".to_owned() }
    }, config);
}
