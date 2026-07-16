use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

use seia::{Engine, SearchClient, config::EngineRegistry};

#[derive(Parser)]
#[command(name = "seia", about = "One query, every search engine.")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Search the web
    Search {
        query: String,

        #[arg(short, long, value_enum, default_value = "duckduckgo")]
        engine: String,

        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Fetch full page content for each result
        #[arg(long)]
        fetch: bool,

        /// Max results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// List available engines
    Engines,

    /// Run the MCP (Model Context Protocol) server on stdio, exposing the
    /// search tools to AI coding assistants.
    #[cfg(feature = "mcp")]
    Mcp,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let registry = EngineRegistry::load().unwrap_or_default();

    match cli.cmd {
        Command::Search {
            query,
            engine,
            json,
            fetch,
            limit,
        } => {
            let eng = parse_engine(&engine);
            let client = SearchClient::new().with_registry(registry);
            let opts = seia::SearchOptions {
                limit: Some(limit),
                fetch_content: fetch,
                searxng_url: None,
            };

            let result = client.search_with_options(&query, eng, opts).await?;

            if json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                println!(
                    "Engine: {} | {} results | {}ms\n",
                    result.engine,
                    result.items.len(),
                    result.elapsed_ms
                );
                for (i, item) in result.items.iter().enumerate() {
                    println!("{}. {}", i + 1, item.title);
                    println!("   {}", item.url);
                    if let Some(snippet) = &item.snippet {
                        println!("   {}", truncate(snippet, 120));
                    }
                    if let Some(content) = &item.content {
                        println!("   [content: {} chars]", content.len());
                    }
                    println!();
                }
            }
        }

        Command::Engines => {
            println!("Built-in engines:");
            println!("  duckduckgo  — Free, HTML scraping, no key needed");
            println!("  wikipedia   — Free, unlimited, academic knowledge");
            println!("  searxng     — Self-hosted meta-search (SEARXNG_URL)");
            println!("  tavily      — AI-optimized API, free 1K/month (TAVILY_API_KEY)");
            println!("  bing        — Bing Web Search API (BING_SEARCH_API_KEY)");
            println!("  brave       — Brave Search API (BRAVE_SEARCH_API_KEY)");
            println!("  zhipu       — 智谱 web_search tool (ZHIPU_API_KEY)");
            println!("  bocha       — 博查 Web Search API (BOCHA_API_KEY)");
            println!("  metaso      — 秘塔 Web Search API (METASO_API_KEY)");
            if !registry.engines.is_empty() {
                println!("\nCustom engines (from config):");
                for (name, def) in &registry.engines {
                    let method = def.method.to_uppercase();
                    println!("  {name}  — {label} [{method}]", label = def.label);
                }
            }
        }

        #[cfg(feature = "mcp")]
        Command::Mcp => {
            seia::mcp::run().await?;
        }
    }

    Ok(())
}

fn parse_engine(raw: &str) -> Engine {
    let lower = raw.to_ascii_lowercase();
    match lower.as_str() {
        "duckduckgo" | "ddg" => Engine::Duckduckgo,
        "tavily" => Engine::Tavily,
        "searxng" => Engine::Searxng,
        "wikipedia" | "wiki" => Engine::Wikipedia,
        "bing" => Engine::Bing,
        "brave" => Engine::Brave,
        "zhipu" => Engine::Zhipu,
        "bocha" => Engine::Bocha,
        "metaso" => Engine::Metaso,
        other => Engine::Custom(other.to_string()),
    }
}

fn truncate(s: &str, max: usize) -> String {
    seia::utils::truncate(s, max)
}
