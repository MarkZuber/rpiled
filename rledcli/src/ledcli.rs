use core::TaskMessage;
use core::TextBlock;

pub struct LedCli {
    client: reqwest::Client,
    base_addr: String,
}

impl LedCli {
    pub fn new(base_addr: &str) -> Self {
        let client = reqwest::Client::new();
        LedCli {
            client,
            base_addr: base_addr.into(),
        }
    }

    pub async fn display_text(&self, text_blocks: &Vec<TextBlock>) -> Result<(), reqwest::Error> {
        self.json_request(&TaskMessage::DisplayText {
            text_blocks: text_blocks.clone(),
        })
        .await
    }

    pub async fn display_image(&self, image_path: &str) -> Result<(), reqwest::Error> {
        self.json_request(&TaskMessage::DisplayImage {
            image_path: image_path.to_string(),
        })
        .await
    }

    pub async fn scroll_text(
        &self,
        text_blocks: &Vec<TextBlock>,
        x_delta: i32,
        y_delta: i32,
        num_steps: i32,
        frame_millis: u64,
    ) -> Result<(), reqwest::Error> {
        self.json_request(&TaskMessage::ScrollText {
            text_blocks: text_blocks.clone(),
            x_delta,
            y_delta,
            num_steps,
            frame_millis,
        })
        .await
    }

    pub async fn draw_circles(&self) -> Result<(), reqwest::Error> {
        self.json_request(&TaskMessage::Circles {}).await
    }

    pub async fn stop_task(&self) -> Result<(), reqwest::Error> {
        self.json_request(&TaskMessage::Stop {}).await
    }

    async fn json_request<REQ: serde::Serialize>(
        &self,
        request: &REQ,
    ) -> Result<(), reqwest::Error> {
        let uri = format!("{}/{}", self.base_addr, "execute");
        self.client.post(&uri).json(&request).send().await?;
        Ok(())
    }
}

pub fn new(base_addr: &str) -> LedCli {
    LedCli::new(base_addr)
}
