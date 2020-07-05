mod ledcli;

use core::TextBlock;
use std::{thread, time};

async fn scroll_text(lcli: &ledcli::LedCli) -> Result<(), reqwest::Error> {
    let mut blocks = vec![
        TextBlock {
            font_path: "/home/pi/rledsvr/content/fonts/4x6.bdf".to_string(),
            text: "Test Message".to_string(),
            x: 10,
            y: 10,
            r: 255,
            g: 0,
            b: 0,
        },
        TextBlock {
            font_path: "/home/pi/rledsvr/content/fonts/5x7.bdf".to_string(),
            text: "Something Else".to_string(),
            x: 10,
            y: 20,
            r: 0,
            g: 0,
            b: 255,
        },
    ];

    for _ in 0..30_i32 {
        lcli.display_text(&blocks).await.unwrap();
        for (_, val) in blocks.iter_mut().enumerate() {
            val.x = val.x - 1
        }
        thread::sleep(time::Duration::from_millis(100));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let lcli = ledcli::new("http://192.168.2.235:8000");

    scroll_text(&lcli).await?;

    lcli.display_text(&vec![
        TextBlock {
            font_path: "/home/pi/rledsvr/content/fonts/4x6.bdf".to_string(),
            text: "Test Message".to_string(),
            x: 10,
            y: 10,
            r: 255,
            g: 0,
            b: 0,
        },
        TextBlock {
            font_path: "/home/pi/rledsvr/content/fonts/5x7.bdf".to_string(),
            text: "Something Else".to_string(),
            x: 10,
            y: 20,
            r: 0,
            g: 0,
            b: 255,
        },
    ])
    .await
    .unwrap();
    thread::sleep(time::Duration::from_secs(10));

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
