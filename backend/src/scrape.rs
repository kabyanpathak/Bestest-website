use reqwest::Client;
use scraper::Html;
use std::io;

pub async fn run() -> bool {
    let client = Client::new();
    let mut website = String::new();

    match io::stdin().read_line(&mut website) {
        Ok(_) => {
            website = website.trim().to_string();
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    if website == "exit" {
        return true;
    }

    let mut content = String::new();
    match client.get(&website).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(html) => {
                        content = html;
                    }
                    Err(_) => {
                        println!("Website Fail");
                    }
                }
            }
        }
        Err(_) => {
            print!("website {} failed", website);
        }
    }
    add_db(&(create(content)));
    false
}

fn add_db(site: &Html) {
    println!("\n--- Full HTML Content ---");
    println!("{}", site.html());
    println!("-------------------------\n");
}

fn create(web: String) -> Html {
    Html::parse_document(&web)
}

/*
 struct Manhwa {
    title: u32,
    titles: Vec<String>,
    authors: Vec<String>,
    genres: Vec<String>,
    synopsis: String,
}
*/
