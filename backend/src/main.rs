use axum::{Router, response::Html, routing::get};
use std::fs;

#[tokio::main]
async fn main() {
    //gets the css file
    let app = Router::new().route("/", get(main_page()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn main_page() -> Html<String> {
    let filepath = "../frontend/mainpage.html";

    match fs::read_to_string(filepath) {
        Ok(html) => Html(html),

        Err(_e) => panic!("you fucked up"),
    }
}
