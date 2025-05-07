use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Record {
    pub id: String,
    pub seq: String,
}
