use reqwest::Client;
use serde_json::{Value, json};
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("OPENAI_KEY").expect("Environment variable OPENAI_KEY not set");
    let endpoint = "https://api.openai.com/v1/engines/davinci-codex/completions";

    let prompt = "Using the Rust programming language, write a program that prints \"Hello, world!\" to the console.";
    let client = Client::new();
    let response = client.post(endpoint)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "prompt": prompt,
            "n": 1,
            "max_tokens": 250,
            "temperature": 0.5
        }))
        .send()
        .await?;

    let result: Value = response.json().await?;
    let rust_code = &result["choices"][0]["text"]
        .as_str()
        .ok_or("No code found in response")?
        .trim();

    let mut file = File::create("hello.rs")?;
    file.write_all(rust_code.as_bytes())?;
    println!("Rust code saved to hello.rs");

    Ok(())
}
