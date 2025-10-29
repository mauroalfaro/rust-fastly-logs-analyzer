use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand, ValueEnum};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde_json::Value;

#[derive(Parser)]
#[command(name = "rust-fastly-logs-analyzer")]
#[command(about = "Query Fastly real-time stats and metrics", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long)]
    token: Option<String>,
    #[arg(long, value_enum, default_value_t = Format::Text)]
    format: Format,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format { Text, Json }

#[derive(Subcommand)]
enum Commands {
    Stats { #[arg(long)] service: String, #[arg(long)] from: Option<String>, #[arg(long)] to: Option<String>, #[arg(long, default_value_t = "minute".to_string())] by: String, #[arg(long, default_value_t=false)] json: bool },
    Summary { #[arg(long)] service: String, #[arg(long, default_value_t=false)] json: bool },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let token = cli.token.or_else(|| std::env::var("FASTLY_TOKEN").ok()).ok_or_else(|| anyhow!("token required via --token or FASTLY_TOKEN"))?;
    let client = Client::builder().build()?;
    match cli.command {
        Commands::Stats { service, from, to, by, json } => {
            let jsonl = matches!(cli.format, Format::Json) || json;
            stats(&client, &token, &service, from.as_deref(), to.as_deref(), &by, jsonl).await?
        }
        Commands::Summary { service, json } => {
            let jsonl = matches!(cli.format, Format::Json) || json;
            summary(&client, &token, &service, jsonl).await?
        }
    }
    Ok(())
}

async fn stats(client: &Client, token: &str, service: &str, from: Option<&str>, to: Option<&str>, by: &str, jsonl: bool) -> Result<()> {
    let mut url = format!("https://api.fastly.com/stats/service/{}?by={}", service, by);
    if let Some(f) = from { url.push_str(&format!("&from={}", encode_time(f)?)); }
    if let Some(t) = to { url.push_str(&format!("&to={}", encode_time(t)?)); }
    let res = client.get(&url).header("Fastly-Key", token).send().await?;
    let val: Value = res.json().await?;
    if jsonl { println!("{}", val); } else { if let Some(arr) = val.pointer("/data").and_then(|v| v.as_array()) { for p in arr { let ts = p.get("start_time").and_then(|x| x.as_i64()).unwrap_or_default(); let req = p.get("requests").and_then(|x| x.as_u64()).unwrap_or_default(); println!("{}\t{}", ts, req); } } }
    Ok(())
}

async fn summary(client: &Client, token: &str, service: &str, jsonl: bool) -> Result<()> {
    let url = format!("https://api.fastly.com/service/{}/stats/summary", service);
    let res = client.get(&url).header("Fastly-Key", token).send().await?;
    let val: Value = res.json().await?;
    if jsonl { println!("{}", val); } else { println!("{}", val); }
    Ok(())
}

fn encode_time(s: &str) -> Result<String> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) { Ok(dt.timestamp().to_string()) } else { Ok(s.to_string()) }
}
