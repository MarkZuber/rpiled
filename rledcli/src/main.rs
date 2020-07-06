#![allow(dead_code)]

extern crate clap;

mod ledcli;

use clap::{App, SubCommand};
use core::TextBlock;
use std::{thread, time};

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
    let matches = App::new("LED PI Client")
        .version("1.0")
        .author("Mark Zuber <mark@zube.com>")
        .about("Displays various content on my Raspberry PI LED board")
        .subcommand(SubCommand::with_name("stop").about("Stops running display code on board"))
        .subcommand(SubCommand::with_name("circles").about("Displays some circles"))
        .subcommand(SubCommand::with_name("scroll").about("Scrolls text"))
        .subcommand(SubCommand::with_name("image").about("Shows Image"))
        .get_matches();

    let lcli = ledcli::new("http://192.168.2.235:8000");

    if let Some(_) = matches.subcommand_matches("stop") {
        lcli.stop_task().await.unwrap();
    } else if let Some(_) = matches.subcommand_matches("circles") {
        lcli.draw_circles().await.unwrap();
    } else if let Some(_) = matches.subcommand_matches("scroll") {
        scroll_text_onboard(&lcli).await.unwrap();
    } else if let Some(_) = matches.subcommand_matches("image") {
        let images = vec![
            "/home/pi/pixelart/lake.png".to_string(),
            "/home/pi/pixelart/pixelnight.png".to_string(),
            "/home/pi/pixelart/restaurant.jpeg".to_string(),
        ];
        for image in images {
            lcli.display_image(&image).await.unwrap();
            thread::sleep(time::Duration::from_millis(5000));
        }
    } else {
        println!("No matching subcommand found, doing nothing...");
    }

    Ok(())
}
