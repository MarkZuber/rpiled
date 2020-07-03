mod ledcli;

use std::{thread, time};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let lcli = ledcli::new("http://192.168.2.235:8000");

    let step_value = 50;

    for i in (0_u8..255_u8).step_by(step_value) {
        for j in (0_u8..255_u8).step_by(step_value) {
            for k in (0_u8..255_u8).step_by(step_value) {
                lcli.draw_circles(i, j, k).await.unwrap();
                thread::sleep(time::Duration::from_millis(500));
            }
        }
    }

    println!("stopping...");
    lcli.stop_task().await.unwrap();

    Ok(())
}
