use core::TaskMessage;

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

    #[allow(dead_code)]
    pub async fn display_text(
        &self,
        font_path: &str,
        message: &str,
        x: i32,
        y: i32,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<(), reqwest::Error> {
        self.json_request(&TaskMessage::DisplayText {
            font_path: font_path.to_string(),
            text: message.to_string(),
            x,
            y,
            r,
            g,
            b,
        })
        .await
    }

    pub async fn draw_circles(&self, r: u8, g: u8, b: u8) -> Result<(), reqwest::Error> {
        self.json_request(&TaskMessage::Circles { r, g, b }).await
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
