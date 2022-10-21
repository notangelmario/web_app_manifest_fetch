use serde_json::{Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url: String = String::from("https://paquet.shop/manifest.json");
    
    let res = reqwest::get(url).await?;

    let data = res.json::<Value>().await?;
    let name = data.get("name").and_then(|value| value.as_str()).unwrap();
    let description = data.get("description").and_then(|value| value.as_str()).unwrap();
    
    println!("App name: {}", name);
    println!("App description: {}", description);
    Ok(())
}