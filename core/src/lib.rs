use serde::{Deserialize, Serialize};

pub mod jobs;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TaskMessage {
    DisplayText {
        font_path: String,
        text: String,
        x: i32,
        y: i32,
        r: u8,
        g: u8,
        b: u8,
    },
    Circles {
        r: u8,
        g: u8,
        b: u8,
    },
    Stop {},
}
