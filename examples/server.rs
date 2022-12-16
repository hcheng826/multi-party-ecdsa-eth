#[macro_use]
extern crate rocket;
mod gg20_signing;
use rocket::serde::{json::Json, Deserialize};
use std::path::PathBuf;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct SignReq {
    msg: String,
    room_id: String,
}

#[post("/", format = "json", data = "<sign_req>")]
async fn sign(sign_req: Json<SignReq>) -> &'static str {
    let _sign_result = gg20_signing::sign(
        sign_req.msg.to_string(),
        PathBuf::from(r"./examples/local-share2.json"),
        vec![1, 2],
        surf::Url::parse("http://localhost:8000").unwrap(),
        sign_req.room_id.to_string(),
    )
    .await;

    "Server Good"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let figment = rocket::Config::figment().merge(("port", 8002));
    let _rocket_instance = rocket::custom(figment)
        .mount("/", routes![index])
        .mount("/sign", routes![sign])
        .launch()
        .await?;
    Ok(())
}
