use std::path::PathBuf;
use anyhow::Result;
use rocket::fs::{FileServer, Options};
use rocket::Request;

#[macro_use] extern crate rocket;

#[catch(404)]
fn catcher() -> &'static str {
    "Oops, we could not load this page. You can try it again in a few minutes lol xD"
}

#[get("/")]
fn index() -> &'static str {
    "<p>Hello this is test</p>"
}

#[get("/api/<name>/<version>")]
fn api(name: &str, version: u32) -> String {
    format!("API Name: {}\nVersion: {}", name, version)
}

#[get("/test/<t>/<amount>")]
fn test(t: &str, amount: usize) -> String {
    let mut s = String::new();
    for i in 0..amount {
        s.push_str(t);
    }
    s
}

#[get("/another_test/<a>/<b>")]
fn another_test(a: usize, b: usize) -> String {
    format!("{} + {} = {}", a, b, a + b)
}

#[get("/file/<name>")]
async fn file(name: &str) -> String {
    name.to_owned()
}

#[get("/combine/<test..>")]
fn combine(test: PathBuf) -> String {
    test.into_os_string().to_str().unwrap().to_owned()
}

#[rocket::main]
async fn main() {
    let ignite = rocket::build()
        .mount("/", routes![index, api, test, another_test, file, combine])
        .mount("/static/", FileServer::new("static/", Options::default()))
        .register("/", catchers![catcher]).launch().await.unwrap();
}