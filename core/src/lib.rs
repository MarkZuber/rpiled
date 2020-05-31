use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DisplayTextRequest {
    pub text: String,
}

#[derive(Deserialize, Serialize)]
pub struct DisplayTextResponse {
    pub message: String,
}
