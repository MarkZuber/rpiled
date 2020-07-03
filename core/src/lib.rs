use serde::{Deserialize, Serialize};

pub mod jobs;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TaskMessage {
    DisplayText { text: String },
    Circles { r: u8, g: u8, b: u8 },
    Stop {},
}
