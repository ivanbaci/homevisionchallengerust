use rayon::prelude::*;
use reqwest;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct House {
    id: u128,
    address: String,
    homeowner: String,
    price: u128,
    photoURL: String,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    houses: Vec<House>,
}

async fn download_photo(house: &House) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(&house.photoURL).await?;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://app-homevision-staging.herokuapp.com/api_project/houses";

    let response = reqwest::get(url).await?;

    let data = response.json::<ApiResponse>().await?;

    data.houses.par_iter().for_each(|house| {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(download_photo(house))
            .unwrap();
    });

    Ok(())
}
