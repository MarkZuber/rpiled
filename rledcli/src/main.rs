// use core::DisplayTextRequest;

mod ledcli;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let lcli = ledcli::new("http://ledpi.local:8000");
    let resp = lcli.display_text("well now we're talking").await.unwrap();
    println!("resp = {:?}", resp.message);

    // let bodycli = lcli.hello("dr zube", 49).await.unwrap();
    // println!("bodycli = {:?}", bodycli);
    // let req = DisplayTextRequest {
    //     text: "hey thats something".into(),
    // };
    // let resp = lcli.display_text_payload(&req).await.unwrap();

    Ok(())
}
