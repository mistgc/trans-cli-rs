use crate::{request::TransBasicReq, config::ParseConfig};
use http;
use crate::config::BasicConfig;
use std::fs;
use toml;
use serde::Deserialize;

#[derive(Debug, Default, PartialEq, Eq, Deserialize)]
struct Config {
    basic: BasicConfig,
}

/// config.toml
/// ```toml
/// [basic]
/// appid = "xxxxxxxx"
/// secert_key = "xxxxxxxx"
/// [ultra]
/// ```
impl ParseConfig for Config {
    fn parse_config(config_path: String) -> Result<Self, std::io::Error>{
        let data = fs::read(config_path)?;
        let config: Self = toml::from_slice(data.as_ref())?;
        Ok(config)
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}


#[derive(Debug, Default, PartialEq, Eq)]
struct BaiduFanyi {
    url: String,
    config: Config,
    text_src: String,
    text_dst: String,
}

impl TransBasicReq for BaiduFanyi {
    fn send_req(&self) -> Vec<u8> {
        todo!()
    }
}


impl BaiduFanyi {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_file(path: String) -> Result<Self, String> {
        let mut config = Config::new();
        if let Ok(conf) = Config::parse_config(path) {
            config = conf;
        } else {
            return Err("[Err] Parsing config failed.".to_owned())
        }
        let origin_url: String = "https://fanyi-api.baidu.com/api/trans/vip/translate".to_owned();

        Ok(Self {
            url: origin_url,
            config,
            ..Self::default()
        })
    }
}

#[test]
fn test_config() {
    let path = "fixtures/test_config.toml";
    let config: Config = Config::parse_config(path.to_owned()).unwrap();
    println!("{:?}", config);
    assert_eq!(Config {
        basic: BasicConfig { appid: "appid_123".to_owned(), secert_key: "key_123".to_owned() }
    }, config);
}

#[test]
fn test_baidu_fanyi() {
    let path = "fixtures/test_config.toml";
    let baidu = BaiduFanyi::from_file(path.to_owned()).unwrap();
    println!("{:?}", baidu);
    assert_eq!(BaiduFanyi {
        url: "https://fanyi-api.baidu.com/api/trans/vip/translate".to_owned(),
        config: Config { basic: BasicConfig { appid: "appid_123".to_owned(), secert_key: "key_123".to_owned() } },
        ..BaiduFanyi::default()
    }, baidu);
}
