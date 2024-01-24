use reqwest;
use serde::Deserialize;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://app-homevision-staging.herokuapp.com/api_project/houses";

    let response = reqwest::get(url).await?;

    let data = response.json::<ApiResponse>().await?;

    println!("{:#?}", data.houses);

    Ok(())
}
