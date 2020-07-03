mod ledcli;

use std::{thread, time};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let lcli = ledcli::new("http://localhost:8000");
    let delay_secs = 1;
    // let phrases = vec![
    //     "well now we're talking",
    //     "some other message",
    //     "more message",
    //     "the last message",
    // ];

    // for phrase in phrases {
    //     println!("Sending: {}", phrase);
    //     lcli.display_text(phrase).await.unwrap();
    //     thread::sleep(time::Duration::from_secs(delay_secs));
    // }

    for i in 0_u8..255_u8 {
        lcli.draw_circles(i, i, i).await.unwrap();
        thread::sleep(time::Duration::from_secs(delay_secs));
    }

    println!("stopping...");
    lcli.stop_task().await.unwrap();

    Ok(())
}
