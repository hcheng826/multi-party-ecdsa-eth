#[macro_use]
extern crate rocket;
use reqwest::Client;
mod gg20_signing;
use std::path::PathBuf;
use tokio::task;
use uuid::Uuid;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", format = "plain", data = "<serialized_tx>")]
async fn send_tx(serialized_tx: &str) -> String {
    let room_id = Uuid::new_v4();

    let heavy = task::spawn(gg20_signing::sign(
        serialized_tx.to_string(),
        PathBuf::from(r"./examples/local-share1.json"),
        vec![1, 2],
        surf::Url::parse("http://localhost:8000").unwrap(),
        room_id.to_string(),
    ));

    let serialized_tx_clone = serialized_tx.to_string();

    let light = task::spawn(async move {
        let client = Client::new();
        let mut body = std::collections::HashMap::new();
        body.insert("msg", serialized_tx_clone);
        body.insert("room_id", room_id.to_string());
        let _res = client
            .post("http://localhost:8002/sign")
            .json(&body)
            .send()
            .expect("REASON")
            .text();
    });

    let (a, _b) = tokio::join!(heavy, light);

    a.unwrap().unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let figment = rocket::Config::figment()
        .merge(("port", 8001));
    let _rocket_instance = rocket::custom(figment)
        .mount("/", routes![index])
        .mount("/send-tx", routes![send_tx])
        .launch()
        .await?;
    Ok(())
}
