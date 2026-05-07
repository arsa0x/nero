#[derive(Debug, serde::Serialize)]
pub struct RequestResult {
    pub name: String,
    pub method: String,
    pub url: String,
    pub status: u16,
    pub duration_ms: u128,
    pub body: String,
}
