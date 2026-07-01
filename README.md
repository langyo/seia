# seia

<div align="center">

**Universal search engine abstraction for Rust**

Multi-backend web search library and CLI — API mode, scrape mode, or headless browser mode.

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](./LICENSE)
[![CI](https://github.com/celestia-island/seia/actions/workflows/checks.yml/badge.svg)](https://github.com/celestia-island/seia/actions)

[English](./README.md) · [简体中文](./docs/zh-hans/README.md)

</div>

## Introduction

seia is a general-purpose search engine abstraction layer. It provides a unified interface to multiple search backends — from free HTML scraping to paid APIs — without requiring you to change your code when switching engines.

**Key design choices:**
- **General-purpose** — not tied to any specific framework. Use as a library or standalone CLI.
- **Multi-backend** — DuckDuckGo (free), Tavily (AI-optimized), SearXNG (self-hosted), Wikipedia (academic), Bing, Brave.
- **Two execution modes** — API mode (calls provider HTTP APIs) and scrape mode (parses HTML search results).
- **Proxy support** — built-in proxy configuration, also auto-detects `HTTP_PROXY`/`HTTPS_PROXY` env vars.
- **Content extraction** — readability-style main text extraction from any URL.

## Quick Start

### CLI

```bash
# Install
cargo install --path .

# Basic search
seia search "rust async patterns"

# Choose engine
seia search "Klein bottle" --engine wikipedia

# JSON output
seia search "climate change" --engine duckduckgo --json

# Fetch page content
seia search "red-black tree implementation" --fetch

# Through a proxy
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# List engines
seia engines
```

### Library

```rust
use seia::{SearchClient, Engine};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = SearchClient::new();
    let results = client.search("rust async patterns", Engine::DuckDuckGo).await?;

    for item in &results.items {
        println!("{}: {}", item.title, item.url);
    }
    Ok(())
}
```

With proxy:

```rust
let client = SearchClient::with_proxy("http://localhost:7890");
```

With content fetch:

```rust
use seia::{SearchClient, Engine, SearchOptions};

let opts = SearchOptions { fetch_content: true, ..Default::default() };
let results = client.search_with_options("deep learning", Engine::DuckDuckGo, opts).await?;
```

Fallback across engines:

```rust
let results = client.search_fallback("topic", &[Engine::Tavily, Engine::DuckDuckGo, Engine::Wikipedia]).await?;
```

## Engines

| Engine | Mode | Auth | Free Tier | How to Get Key |
|--------|------|------|-----------|----------------|
| **DuckDuckGo** | Scrape | None | Unlimited | N/A |
| **Wikipedia** | API | None | Unlimited | N/A |
| **SearXNG** | API | None | Self-hosted | [Deploy your own](https://searxng.github.io/searxng/admin/installation.html) |
| **Tavily** | API | `TAVILY_API_KEY` | 1,000/month | [app.tavily.com](https://app.tavily.com) — sign up, copy API key |
| **Bing** | API | `BING_SEARCH_API_KEY` | 1,000/month | [Azure Portal](https://portal.azure.com) → Create "Bing Search v7" resource |
| **Brave** | API | `BRAVE_SEARCH_API_KEY` | 2,000/month | [brave.com/search/api](https://brave.com/search/api/) — register, get API key |

### Setting API Keys

```bash
# Option 1: Environment variable
export TAVILY_API_KEY=tvly-xxxxxxxxxxxxx

# Option 2: In .env file
echo 'TAVILY_API_KEY=tvly-xxxxxxxxxxxxx' >> .env

# Option 3: In your shell profile (~/.bashrc / ~/.zshrc)
echo 'export TAVILY_API_KEY=tvly-xxxxxxxxxxxxx' >> ~/.bashrc
```

### SearXNG Setup

```bash
# Docker one-liner
docker run -d -p 8080:8080 --name searxng searxng/searxng

# Then set the URL
export SEARXNG_URL=http://localhost:8080

# Search
seia search "test query" --engine searxng
```

## Proxy Support

seia supports HTTP/HTTPS/SOCKS proxies via two mechanisms:

1. **Automatic** — set `HTTP_PROXY` / `HTTPS_PROXY` environment variables. reqwest picks them up automatically.
2. **Explicit** — use `SearchClient::with_proxy("http://localhost:7890")` in code.

```bash
# CLI with proxy
HTTPS_PROXY=http://localhost:7890 seia search "query"

# Or for all commands
export HTTPS_PROXY=http://localhost:7890
seia search "query"
```

## Development

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
just fmt         # cargo fmt
just clippy      # cargo clippy -D warnings
```

## License

SySL-1.0 (Synthetic Source License).
