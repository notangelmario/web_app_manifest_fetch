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

    let name = data.get("name")
        .and_then(|value| value.as_str()).unwrap_or_else(|| "No name");
    let description = data.get("description")
        .and_then(|value| value.as_str()).unwrap_or_else(|| "No description");
    
    print!("\n");
    println!("App name: {}", name);
    println!("App description: {}", description);
    Ok(())
}