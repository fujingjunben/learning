use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    log: String,
}

/// extract `Info` using serde
fn index(info: web::Json<Info>) -> Result<String> {
    println!("{}", info.log);
    Ok(format!("Welcome {}!", info.log))
}

fn main() {
    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind("192.168.0.63:8089")
        .unwrap()
        .run()
        .unwrap();
}
