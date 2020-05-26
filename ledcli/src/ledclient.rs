use std::io::prelude::*;
use std::net::TcpStream;

pub struct LedClient {
    target_addr: String,
    tcp_stream: Option<TcpStream>,
}

impl LedClient {
    pub fn new(target_addr: &str) -> Self {
        Self {
            target_addr: String::from(target_addr),
            tcp_stream: None,
        }
    }

    pub fn connect(&mut self) -> std::io::Result<()> {
        let stream = TcpStream::connect(&self.target_addr)?;
        self.tcp_stream = Some(stream);
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.tcp_stream = None;
    }
}

impl Drop for LedClient {
    fn drop(&mut self) {
        self.disconnect();
    }
}
