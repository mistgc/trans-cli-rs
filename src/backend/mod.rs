pub(crate) mod baidu_trans;
pub(crate) mod youdao_trans;
use reqwest;

use std::fmt::Debug;

pub(crate) trait Backend where Self: Debug {
    fn send_req(&mut self, from: &String, to: &String, text: String) -> Result<String, reqwest::Error>;

    fn handle_response(&self, resp: String) -> String;
}
