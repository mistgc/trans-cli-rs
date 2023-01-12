use crate::backend;
use md5;
use rand::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub(crate) struct Backend {
    pub(crate) url: String,

    appid: String,

    secret_key: String,
}

#[derive(Deserialize, Serialize)]
struct TransResult {
    pub src: String,

    pub dst: String,
}

#[derive(Deserialize, Serialize)]
struct TransError {
    pub error_code: String,

    pub error_msg: String,
}

#[derive(Deserialize, Serialize)]
struct Response {
    pub from: String,

    pub to: String,

    pub trans_result: Vec<TransResult>,
}

impl Backend {
    pub fn new(appid: &str, secret_key: &str) -> Self {
        Self {
            url: "".to_string(),
            appid: appid.to_string(),
            secret_key: secret_key.to_string(),
        }
    }

    fn format_url(&mut self, from: &String, to: &String, text: &String) {
        let salt: i32 = random();
        let sign: String = format!(
            "{:x}",
            md5::compute(
                self.appid.clone() + text + salt.to_string().as_str() + self.secret_key.as_str()
            )
        );
        self.url = format!("https://fanyi-api.baidu.com/api/trans/vip/translate?q={}&from={}&to={}&appid={}&salt={}&sign={}",
                           text,
                           from,
                           to,
                           self.appid.as_str(),
                           salt.to_string().as_str(),
                           sign.as_str());
    }
}

impl backend::Backend for Backend {
    fn send_req(
        &mut self,
        from: &String,
        to: &String,
        text: String,
    ) -> Result<String, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        self.format_url(from, to, &text);
        let resp = client.post(&self.url).send()?.text()?;

        Ok(resp)
    }

    fn handle_response(&self, resp: String) -> String {
        let mut result = "".to_owned();
        if let Ok(resp) = serde_json::from_str::<Response>(resp.as_str()) {
            result = resp.trans_result[0].dst.clone();
        } else {
            let err: TransError = serde_json::from_str(resp.as_str()).unwrap();
            result = format!(
                "Error Code: {}\nError Message: {}",
                err.error_code, err.error_msg
            );
        }

        result
    }
}
