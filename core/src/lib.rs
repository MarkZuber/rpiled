use serde::{Deserialize, Serialize};

pub mod jobs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextBlock {
    pub font_path: String,
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TaskMessage {
    DisplayText {
        text_blocks: Vec<TextBlock>,
    },
    ScrollText {
        text_blocks: Vec<TextBlock>,
        x_delta: i32,
        y_delta: i32,
        num_steps: i32,
        frame_millis: u64,
    },
    Circles {},
    Stop {},
}
