# Engines

seia supports 10 backends through two access paths:

- **API / scrape engines** (6): in the `Engine` enum, usable from both CLI
  and library — just pick an `Engine` variant.
- **Browser engines** (4): CLI-only, driven through the `--browser` flag
  via [tairitsu](https://github.com/celestia-island/tairitsu). They are
  *not* `Engine` variants; the CLI resolves profile names.

Most search engines have official REST APIs with API keys (Google Custom
Search, Bing Web Search, Brave Search, etc.). The browser profiles are a
workaround for engines where the API backend hasn't been implemented yet,
OR where the API is not freely available (Google CSE charges per query).
When an API backend ships, the browser profile for that engine becomes
optional.

## Three execution modes

| Mode | How it works | Used by |
| --- | --- | --- |
| **API** | Calls a search provider's HTTP API, parses JSON. | Tavily, SearXNG, Wikipedia |
| **Scrape** | Fetches the lightweight HTML results page, extracts hits. | DuckDuckGo |
| **Browser** | Drives a headless browser to render JS-heavy pages, extracts via CSS selectors. | Google, Baidu, Bing (web), Yandex |

API and scrape modes need nothing but an HTTP client. Browser mode is
described in [Browser Mode](./browser-mode.md).

## Engine matrix

### API / scrape (in the `Engine` enum — CLI + library)

| Engine | Enum | Mode | Auth | Free tier | Status |
| --- | --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | Scrape | none | unlimited | ✅ |
| Wikipedia | `Wikipedia` | API | none | unlimited | ✅ |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | self-hosted | ✅ |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000/month | ✅ |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000/month | 🔜 |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000/month | 🔜 |

> Bing and Brave API backends are stubs — they return "not yet implemented".
> Use the browser profiles as a stopgap, or [contribute](https://github.com/celestia-island/seia) the API integration.

### Browser (CLI-only — `seia search --browser --engine <name>`)

| Profile | `--engine` value | Description |
| --- | --- | --- |
| google | `google` | Google web search (free scraping, no key; has a [paid CSE API](https://developers.google.com/custom-search)) |
| baidu | `baidu` | Baidu web search |
| bing_web | `bing_web` | Bing web results (Bing also has a [paid Search API](https://www.microsoft.com/en-us/bing/apis/bing-web-search-api)) |
| yandex | `yandex` | Yandex web search |

The browser profiles *mirror* the free, no-key scraping path. Each of these
engines also has an official REST API with an API key — the long-term
plan is to implement `Engine` variants for them so they become first-class
backends, at which point the browser profile becomes a fallback for when
the API key isn't available.

## Selecting an engine

CLI:

```bash
seia search "query" --engine wikipedia           # API (Engine enum)
seia search "query" --engine google --browser    # browser profile
```

Library:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;     // API
// Browser engines aren't available through the library — use BrowserClient + SearchProfile.
```

## Inspecting engine metadata

`Engine` carries its own metadata:

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing
    println!("  needs key? {}", engine.needs_key());
    println!("  key env:    {:?}", engine.api_key_env());
}
```
