//! Search engine profiles for browser mode.
//!
//! Each profile defines the CSS selectors and URL template for a specific
//! search engine's HTML results page. Used by the browser backend to
//! navigate and extract results.

use serde::{Deserialize, Serialize};

/// A search engine profile for browser-based scraping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableProfile {
    pub name: String,
    pub result_selector: String,
    pub title_selector: String,
    pub link_selector: String,
    pub snippet_selector: Option<String>,
    pub wait_selector: String,
}

/// A search engine profile for browser-based scraping.
#[derive(Debug, Clone)]
pub struct SearchProfile {
    /// Engine name (e.g. "google", "baidu").
    pub name: &'static str,
    /// Builds the search URL from a query.
    pub search_url: fn(query: &str) -> String,
    /// CSS selector for result containers (each match = one result).
    pub result_selector: &'static str,
    /// CSS selector for result titles (within a container).
    pub title_selector: &'static str,
    /// CSS selector for result links (within a container).
    pub link_selector: &'static str,
    /// CSS selector for result snippets (within a container, optional).
    pub snippet_selector: Option<&'static str>,
    /// Selector to wait for before extracting results.
    pub wait_selector: &'static str,
}

/// Google search profile.
pub const GOOGLE: SearchProfile = SearchProfile {
    name: "google",
    search_url: |q| format!("https://www.google.com/search?q={}", urlencode(q)),
    // Google 2025-2026 layout: div.g contains each result
    result_selector: "div.g",
    title_selector: "h3",
    link_selector: "a",
    snippet_selector: Some("div[data-sncf], div.VwiC3b, span.aCOpRe"),
    wait_selector: "div.g",
};

/// Baidu search profile.
pub const BAIDU: SearchProfile = SearchProfile {
    name: "baidu",
    search_url: |q| format!("https://www.baidu.com/s?wd={}", urlencode(q)),
    result_selector: "div.result, div.c-container",
    title_selector: "h3",
    link_selector: "h3 a",
    snippet_selector: Some("span.content-right_8Zs40, div.c-abstract"),
    wait_selector: "div.result, div.c-container",
};

/// Bing (web, not API) search profile.
pub const BING_WEB: SearchProfile = SearchProfile {
    name: "bing_web",
    search_url: |q| format!("https://www.bing.com/search?q={}", urlencode(q)),
    result_selector: "li.b_algo",
    title_selector: "h2",
    link_selector: "h2 a",
    snippet_selector: Some("p.b_lineclamp, div.b_caption"),
    wait_selector: "li.b_algo",
};

/// Yandex search profile.
pub const YANDEX: SearchProfile = SearchProfile {
    name: "yandex",
    search_url: |q| format!("https://yandex.com/search/?text={}", urlencode(q)),
    result_selector: "li.serps-item, div.Organic",
    title_selector: "h2, div.OrganicTitle",
    link_selector: "a.OrganicTitle-Link, a.link",
    snippet_selector: Some("div.TextContainer, div.OrganicText"),
    wait_selector: "li.serps-item, div.Organic",
};

/// Get a profile by name.
pub fn get_profile(name: &str) -> Option<&'static SearchProfile> {
    match name {
        "google" => Some(&GOOGLE),
        "baidu" => Some(&BAIDU),
        "bing_web" => Some(&BING_WEB),
        "yandex" => Some(&YANDEX),
        _ => None,
    }
}

fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{:02X}", byte)),
        }
    }
    out
}
