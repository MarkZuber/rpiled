use core::{DisplayTextRequest, DisplayTextResponse};

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

    // pub async fn hello(&self, name: &str, age: i32) -> Result<String, reqwest::Error> {
    //     let uri = format!("{}/hello/{}/{}", self.base_addr, name, age);
    //     let res = self.client.get(&uri).send().await?;
    //     let body = res.text().await?;
    //     Ok(body)
    // }

    pub async fn display_text(&self, message: &str) -> Result<DisplayTextResponse, reqwest::Error> {
        let req = DisplayTextRequest {
            text: message.into(),
        };

        self.display_text_payload(&req).await
    }

    pub async fn display_text_payload(
        &self,
        request: &DisplayTextRequest,
    ) -> Result<DisplayTextResponse, reqwest::Error> {
        self.json_request("displaytext", request).await
    }

    async fn json_request<REQ: serde::Serialize, RESP: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        request: &REQ,
    ) -> Result<RESP, reqwest::Error> {
        let uri = format!("{}/{}", self.base_addr, method);
        let res = self.client.post(&uri).json(&request).send().await?;
        let response: RESP = res.json().await?;
        Ok(response)
    }
}

pub fn new(base_addr: &str) -> LedCli {
    LedCli::new(base_addr)
}
