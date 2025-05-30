use axum::{Router, response::Html, routing::get};
use std::fs;
mod scrape;

#[tokio::main]
async fn main() {
    loop {
        let val: bool = scrape::run().await;
        if val {
            break;
        }
    }

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();

    axum::serve(listener, routeer()).await.unwrap();
}

fn routeer() -> Router {
    Router::new().route("/", get(main_page()))
}

fn main_page() -> Html<String> {
    let filepath = "../frontend/mainpage.html";

    match fs::read_to_string(filepath) {
        Ok(html) => Html(html),

        Err(_e) => panic!("you fucked up"),
    }
}
