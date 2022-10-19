/// Interface return response by bytes for concrete struct.
pub trait TransBasicReq {
    fn send_req(&self) -> Vec<u8>;
}
