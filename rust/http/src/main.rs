use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Deserialize)]
struct Info {
    log: String,
    time: String,
}

fn write(content: &str) {
    let mut file = match OpenOptions::new().append(true).open("log.txt") {
        Err(why) => File::create("log.txt").unwrap(),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => println!("write log error: {}", why.description()),
        Ok(_) => (),
    }
}

/// extract `Info` using serde
fn index(info: web::Json<Info>) -> Result<String> {
    let content = format!("{}: {}\n", info.time, info.log);
    write(&content);
    println!("{}", content);
    Ok("".to_string())
}

fn main() {
    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind("192.168.0.63:8088")
        .unwrap()
        .run()
        .unwrap();
}
