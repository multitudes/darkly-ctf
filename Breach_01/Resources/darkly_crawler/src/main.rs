use reqwest;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

const BASE_URL: &str = "http://localhost:8080/.hidden/";

const TROLL_MESSAGES: &[&str] = &[
    "Non ce n'est toujours pas bon ...",
    "Tu veux de l'aide ? Moi aussi !",
    "Demande à ton voisin du dessous",
    "Demande à ton voisin de gauche",
    "Toujours pas tu vas craquer non ?",
    "Demande à ton voisin de droite",
    "Demande à ton voisin du dessus",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let visited = Arc::new(Mutex::new(HashSet::new()));
    println!("🚀 Starting high-speed Rust hunt...");
    
    crawl(BASE_URL.to_string(), Arc::clone(&visited)).await?;
    Ok(())
}

#[async_recursion::async_recursion]
async fn crawl(url: String, visited: Arc<Mutex<HashSet<String>>>) -> Result<(), Box<dyn std::error::Error>> {
    {
        let mut v = visited.lock().await;
        if v.contains(&url) { return Ok(()); }
        v.insert(url.clone());
    }

    let res = reqwest::get(&url).await?.text().await?;
    
    // We create a scope or just make sure we extract strings quickly
    let mut links_to_visit = Vec::new();
    {
        let document = Html::parse_document(&res);
        let selector = Selector::parse("a").unwrap();

        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                if href == "../" || href.contains("index.html") { continue; }
                
                let full_url = if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("{}{}", url, href)
                };
                // Store the string, NOT the ElementRef
                links_to_visit.push((href.to_string(), full_url));
            }
        }
    } // 'document' and all 'ElementRef's are dropped here!

    for (href, full_url) in links_to_visit {
        if href == "README" {
            let content = reqwest::get(&full_url).await?.text().await?;
            let c = content.trim();
            if !TROLL_MESSAGES.contains(&c) {
                println!("\n🎯 FLAG FOUND at: {}", full_url);
                println!("Content: {}\n", c);
            }
        } else if href.ends_with('/') {
            crawl(full_url, Arc::clone(&visited)).await?;
        }
    }
    Ok(())
}