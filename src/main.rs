use clap::Parser;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

mod utils;
use crate::utils::wrap_text;


/// CLI for querying SearXNG
#[derive(Parser)]
#[command(name = "searcli", version = "1.0", about = "CLI tool to query a SearXNG instance")]
struct Cli {
    /// Search query
    query: String,

    /// SearXNG instance URL
    #[arg(short, long, default_value = "https://search.notashelf.dev")]
    instance: String,

    /// Number of results per page
    #[arg(short, long, default_value_t = 10)]
    limit: usize,

    /// Search category (e.g., general, news, images)
    #[arg(short, long, default_value = "general")]
    category: String,

    /// Max line length for description
    #[arg(long, default_value_t = 80)]
    max_line_length: usize,
}

#[derive(Deserialize, Debug)]
struct SearchResult {
    title: String,
    url: String,
    content: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SearXNGResponse {
    results: Vec<SearchResult>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let client = Client::new();
    let url = format!(
        "{}/search?q={}&format=json&categories={}",
        args.instance, args.query, args.category
    );

    let res = client.get(&url).send().await?;
    let body: SearXNGResponse = res.json().await?;

    println!("\n════════════════════════════════════");
    println!("🔍 Query: {}  |  Category: {}", args.query, args.category);
    println!("════════════════════════════════════");

    for (i, result) in body.results.iter().take(args.limit).enumerate() {
        let wrapped_content = result
            .content
            .as_deref()
            .map(|content| wrap_text(content, args.max_line_length))
            .unwrap_or_else(|| "No description available".to_string());

        let wrapped_content_lines: Vec<String> = wrapped_content
            .lines()
            .map(|line| format!("    {}", line))
            .collect();

        let aligned_content = wrapped_content_lines.join("\n");

        println!(
            "{}️⃣ \x1b[1;34m{}\x1b[0m\n    🌐  \x1b[4;32m{}\x1b[0m\n    📝 {}\n------------------------------------",
            i + 1,
            result.title,
            result.url,
            aligned_content
        );
    }

    Ok(())
}

