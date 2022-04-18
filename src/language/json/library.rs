use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Library {
    pub name: String,
    pub version: String,
    pub source: Option<String>,
}
