use reqwest::Client;
use scraper::Html;
use std::io;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let mut website = String::new();

    match io::stdin().read_line(&mut website) {
        Ok(_) => {
            website = website.trim().to_string();
        }
        Err(e) => {
            print!("{}", e);
        }
    }

    let response = client.get(website).send().await;

    match client.get(&website).send().await {
        Err(_) {
            print!("website {} failed", website);
        }
    }
}

fn create(web: String) -> Html<String> {}

struct Manhwa {
    title: u32,
    titles: Vec<String>,
    authors: Vec<String>,
    genres: Vec<String>,
    synopsis: String,
}
