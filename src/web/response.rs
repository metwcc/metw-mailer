use serde::Serialize;

#[derive(Serialize)]
pub struct Uptime {
    pub uptime: u64,
    pub message: String,
}

#[derive(Serialize)]
pub struct SendMail {
    pub success: bool,
    pub message: String,
}
