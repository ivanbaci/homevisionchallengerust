use reqwest;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio;

#[derive(Clone, Deserialize, Debug)]
struct House {
    id: u128,
    address: String,
    #[serde(rename = "photoURL")]
    photo_url: String,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    houses: Vec<House>,
}

async fn download_photo(house: House) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(house.photo_url).await?;
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let file_path = Path::new(&manifest_dir)
        .join("photos")
        .join(format!("{}_{}.jpg", house.id, house.address));

    let mut file = File::create(file_path)?;

    let content = response.bytes().await?;
    file.write_all(&content)?;
    println!("Photo of house {} downloaded", house.id);
    Ok(())
}

async fn fetch_houses(url: String) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    loop {
        let response = reqwest::get(&url).await?;

        if response.status().is_success() {
            let data = response.json::<ApiResponse>().await?;
            return Ok(data);
        } else {
            eprintln!(
                "Error getting data (Status: {}), retrying...",
                response.status()
            );
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Uso: {} <page> <per_page>", args[0]);
        return Ok(());
    }

    let page = &args[1];
    let per_page = &args[2];
    let base_url = "http://app-homevision-staging.herokuapp.com/api_project/houses";
    let url = format!("{}?page={}&perPage={}", base_url, page, per_page);

    let data = fetch_houses(url).await?;

    let mut handles = Vec::new();

    for house in data.houses {
        let house_clone = house.clone();
        let handle = tokio::spawn(async move {
            download_photo(house_clone).await.unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }
    Ok(())
}
