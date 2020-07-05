mod ledcli;

use core::TextBlock;
use std::{thread, time};

// async fn scroll_text(lcli: &ledcli::LedCli) -> Result<(), reqwest::Error> {
//     let mut blocks = vec![
//         TextBlock {
//             font_path: "/home/pi/rledsvr/content/fonts/4x6.bdf".to_string(),
//             text: "Test Message".to_string(),
//             x: 64,
//             y: 10,
//             r: 255,
//             g: 0,
//             b: 0,
//         },
//         TextBlock {
//             font_path: "/home/pi/rledsvr/content/fonts/5x7.bdf".to_string(),
//             text: "Something Else".to_string(),
//             x: 64,
//             y: 20,
//             r: 0,
//             g: 0,
//             b: 255,
//         },
//         TextBlock {
//             font_path: "/home/pi/rledsvr/content/fonts/5x7.bdf".to_string(),
//             text: "And in green...".to_string(),
//             x: 64,
//             y: 30,
//             r: 0,
//             g: 255,
//             b: 0,
//         },
//     ];

//     for _ in 0..128_i32 {
//         lcli.display_text(&blocks).await.unwrap();
//         for (_, val) in blocks.iter_mut().enumerate() {
//             val.x = val.x - 1
//         }
//         thread::sleep(time::Duration::from_millis(75));
//     }

//     Ok(())
// }

async fn scroll_text_onboard(lcli: &ledcli::LedCli) -> Result<(), reqwest::Error> {
    let blocks = vec![
        TextBlock {
            font_path: "/home/pi/rledsvr/content/fonts/4x6.bdf".to_string(),
            text: "Test Message".to_string(),
            x: 64,
            y: 10,
            r: 255,
            g: 0,
            b: 0,
        },
        TextBlock {
            font_path: "/home/pi/rledsvr/content/fonts/5x7.bdf".to_string(),
            text: "Something Else".to_string(),
            x: 64,
            y: 20,
            r: 0,
            g: 0,
            b: 255,
        },
        TextBlock {
            font_path: "/home/pi/rledsvr/content/fonts/5x7.bdf".to_string(),
            text: "And in green...".to_string(),
            x: 64,
            y: 30,
            r: 0,
            g: 255,
            b: 0,
        },
    ];

    lcli.scroll_text(&blocks, -1, 0, 128, 60).await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let lcli = ledcli::new("http://192.168.2.235:8000");

    scroll_text_onboard(&lcli).await?;

    // for _ in 0..10 {
    //     scroll_text(&lcli).await?;
    // }

    // let step_value = 50;

    // for i in (0_u8..255_u8).step_by(step_value) {
    //     for j in (0_u8..255_u8).step_by(step_value) {
    //         for k in (0_u8..255_u8).step_by(step_value) {
    //             lcli.draw_circles(i, j, k).await.unwrap();
    //             thread::sleep(time::Duration::from_millis(500));
    //         }
    //     }
    // }

    // println!("stopping...");
    // lcli.stop_task().await.unwrap();

    Ok(())
}
