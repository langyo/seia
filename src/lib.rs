//! seia — Universal Search Engine Abstraction
//!
//! Multi-backend web search library and CLI. Supports both API-based and
//! browser-scraping search modes through a unified interface.

pub mod engines;
pub mod engines_impl;
pub mod client;
pub mod browser;
pub mod profiles;
pub mod extractor;
pub mod result;

pub use client::{SearchClient, SearchOptions};
pub use result::{SearchResult, SearchItem, SearchMode};
pub use engines::Engine;
pub use browser::BrowserClient;
pub use profiles::{SearchProfile, get_profile};

/// Embedded browser server (requires `embedded-browser` feature).
/// Spawns tairitsu's debug server in-process so BrowserClient can connect
/// without a separate daemon.
#[cfg(feature = "embedded-browser")]
pub mod embedded {
    use std::sync::OnceLock;

    /// Start the embedded tairitsu debug server in a background task.
    /// Returns the endpoint URL to pass to `BrowserClient::new`.
    pub fn start(port: u16, proxy: Option<&str>) -> anyhow::Result<String> {
        static STARTED: OnceLock<()> = OnceLock::new();

        if STARTED.get().is_some() {
            return Ok(format!("http://127.0.0.1:{}", port));
        }

        let cfg = tairitsu_packager::debug::DebugServerConfig {
            base_url: "about:blank".to_string(),
            dev_port: 0,
            dist_dir: "(seia-embedded)".to_string(),
            package_name: "seia".to_string(),
            proxy: proxy.map(|p| p.to_string()),
        };

        tokio::spawn(async move {
            let _ = tairitsu_packager::debug::start_debug_server(cfg, port).await;
        });

        STARTED.set(()).ok();
        Ok(format!("http://127.0.0.1:{}", port))
    }
}

pub mod prelude {
    pub use crate::{SearchClient, Engine, SearchResult, SearchItem, SearchMode};
}
