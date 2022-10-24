use crate::backend;
use reqwest;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Default)]
pub(crate) struct Backend {
    pub(crate) url: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct TranslateResult {
    src: String,

    tgt: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Response {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    trans_type: String,

    errorCode: i32,

    elapsedTime: i32,

    translateResult: Vec<Vec<TranslateResult>>,
}

impl Backend {
    pub(crate) fn new(_appid: &str, _secret_key: &str) -> Self {
        Self::default()
    }

    fn format_url(&mut self, from: &String, to: &String, text: &String) {
        let trans_type = match (from.as_str(), to.as_str()) {
            // use auto mode
            (_, _) => "AUTO",
        };
        self.url = format!("https://fanyi.youdao.com/translate?&doctype=json&type={}&i={}", trans_type, text);
    }
}

impl backend::Backend for Backend {
    fn send_req(&mut self, from: &String, to: &String, text: String) -> Result<String, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        self.format_url(from, to, &text);
        let resp = client.get(&self.url).send()?.text()?;

        Ok(resp)
    }

    fn handle_response(&self, resp: String) -> String {
        let mut result = "".to_owned();
        let resp: Response = serde_json::from_str(resp.as_str()).unwrap();
        result = resp.translateResult[0][0].tgt.clone();
        result
    }
}
