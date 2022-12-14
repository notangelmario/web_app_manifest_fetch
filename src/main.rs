use serde_json::Value;
use std::process;
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    print!("Input a manifest.json URL: ");
    // We dump the result of stdout().flush()
    // to please the compiler
    let _ = stdout().flush();

    stdin().read_line(&mut line).unwrap_or_else(|_| {
        eprintln!("Could not get input!");
        process::exit(1)
    });
    
    let res = reqwest::get(line)
        .await
        .unwrap_or_else(|_| {
            eprintln!("Could not fetch URL!");
            process::exit(1)
        });


    let data = res.json::<Value>()
        .await
        .unwrap_or_else(|_| {
            eprintln!("Could not get JSON data!");
            process::exit(1)
        });

    let name = data.get("name").and_then(|value| value.as_str());
    let short_name = data.get("short_name").and_then(|value| value.as_str());
    let description = data.get("description").and_then(|value| value.as_str());
    let categories = data.get("categories")
        .and_then(|value| {
            value.as_array()
                .unwrap()
                .iter()
                .map(|val| val.as_str())
                .collect::<Option<Vec<&str>>>()
        });

    print!("\n");
    println!("App name: {}", name.unwrap_or("No name"));
    println!("App short name: {}", short_name.unwrap_or("No short name"));
    println!("App description: {}", description.unwrap_or("No description"));
    println!("App categories: {}", categories.unwrap_or(vec!["No categories"]).join(", "));
    Ok(())
}
