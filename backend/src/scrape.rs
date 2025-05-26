use std::io;

use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let mut website = String::new();
    match io::stdin().read_line(&mut website) {
        Ok(_) => {
            website.trim();
        }
        Err(e) => {
            print!("{}", e);
        }
    }

    let response = client.get(website).send().await.unwrap();
}

struct Manhwa {
    title: u32,
    titles: Vec<String>,
    authors: Vec<String>,
    genres: Vec<String>,
    synopsis: String,
}
