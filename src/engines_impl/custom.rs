//! Generic custom engine — drives any HTTP search endpoint via TOML config.
//!
//! Unlike the hand-written engine modules, this one is entirely
//! data-driven. It reads a [`CustomEngineDef`] from the config registry,
//! constructs an HTTP request using the template rules, and maps the
//! JSON response into [`SearchItem`]s via JSONPath + field selectors.

use anyhow::{Context as _, Result, anyhow};
use jsonpath_rust::JsonPath;
use serde_json::Value;

use crate::{
    client::SearchOptions,
    config::CustomEngineDef,
    engines_impl::EngineOutput,
    result::{SearchItem, SearchMode},
};

/// Execute a search against a user-defined custom engine.
///
/// # Errors
///
/// Returns `Err` when the HTTP request fails, the response cannot be parsed
/// as JSON, or the JSONPath/field selectors yield no usable results.
pub async fn search(
    http: &reqwest::Client,
    query: &str,
    opts: &SearchOptions,
    def: &CustomEngineDef,
    _engine_name: &str,
) -> Result<EngineOutput> {
    let limit = opts.limit.unwrap_or(10);
    let rendered_url = def.render(&def.url, query, limit);

    let method = def.method.to_uppercase();
    let mut req = match method.as_str() {
        "GET" => {
            let mut url = rendered_url;
            if let Some(ref qp) = def.query_param {
                let encoded = crate::utils::urlencode_query(query);
                let sep = if url.contains('?') { "&" } else { "?" };
                url = format!("{url}{sep}{qp}={encoded}");
            }
            if let Some(ref lp) = def.limit_param {
                let sep = if url.contains('?') { "&" } else { "?" };
                url = format!("{url}{sep}{lp}={limit}");
            }
            http.get(&url)
        }
        "POST" => {
            let body = def
                .body_template
                .as_ref()
                .map(|b| def.render(b, query, limit))
                .unwrap_or_else(|| {
                    serde_json::json!({"query": query, "limit": limit}).to_string()
                });
            http.post(&rendered_url).body(body)
        }
        other => return Err(anyhow!("unsupported HTTP method: {other}")),
    };

    // Apply headers with env-var substitution.
    for (k, v) in &def.headers {
        let rendered = def.render(v, query, limit);
        req = req.header(k.as_str(), &rendered);
    }

    // ── Pre-request script (optional, requires boa) ──
    #[cfg(feature = "pre-request-script")]
    if let Some(ref script) = def.pre_request {
        req = run_pre_request_script(req, script, query, limit, &rendered_url, &method)?;
    }
    #[cfg(not(feature = "pre-request-script"))]
    if def.pre_request.is_some() {
        return Err(anyhow!(
            "pre_request script requires the `pre-request-script` feature (cargo install seia --features pre-request-script)"
        ));
    }

    let resp = req.send().await.context("HTTP request failed")?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(anyhow!("HTTP {status}: {}", truncate(&body, 400)));
    }

    let json: Value = resp
        .json()
        .await
        .context("parsing JSON response")?;

    let items = extract_items(&json, def).context("extracting search results")?;

    Ok((items, SearchMode::Api))
}

/// Run the JSONPath query and map each element to a `SearchItem`.
fn extract_items(json: &Value, def: &CustomEngineDef) -> Result<Vec<SearchItem>> {
    let results: Vec<Value> = if let Some(ref path) = def.result_path {
        let path = JsonPath::<Value>::try_from(path.as_str())
            .map_err(|e| anyhow!("invalid JSONPath '{}': {e}", path))?;
        let found = path.find(json);
        if found.is_array() {
            found.as_array().unwrap().to_vec()
        } else {
            vec![found]
        }
    } else if let Some(arr) = json.as_array() {
        arr.clone()
    } else {
        vec![json.clone()]
    };

    let items: Vec<SearchItem> = results
        .iter()
        .filter_map(|item| {
            let title = dot_get(item, &def.title_field)?;
            let url = dot_get(item, &def.url_field)?;
            let snippet = def.snippet_field.as_ref().and_then(|f| dot_get(item, f));
            Some(SearchItem {
                title,
                url,
                snippet,
                content: None,
            })
        })
        .collect();

    if items.is_empty() {
        return Err(anyhow!("no results matched the field selectors"));
    }
    Ok(items)
}

/// Extract a value from a JSON object by dot-separated path (e.g.
/// `repository.full_name`). Returns `None` if any segment is missing.
fn dot_get(value: &Value, path: &str) -> Option<String> {
    let mut cur = value;
    for seg in path.split('.') {
        cur = cur.get(seg)?;
    }
    match cur {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => Some(cur.to_string()),
    }
}

fn truncate(s: &str, max: usize) -> String {
    crate::utils::truncate(s, max)
}

// ── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_get_simple() {
        let json: Value = serde_json::from_str(r#"{"a": {"b": "c"}}"#).unwrap();
        assert_eq!(dot_get(&json, "a.b").as_deref(), Some("c"));
    }

    #[test]
    fn dot_get_missing() {
        let json: Value = serde_json::from_str(r#"{"a": 1}"#).unwrap();
        assert_eq!(dot_get(&json, "x"), None);
    }

    #[test]
    fn dot_get_nested_missing() {
        let json: Value = serde_json::from_str(r#"{"a": {"b": 1}}"#).unwrap();
        assert_eq!(dot_get(&json, "a.b.c"), None);
    }

    #[test]
    fn dot_get_number_to_string() {
        let json: Value = serde_json::from_str(r#"{"count": 42}"#).unwrap();
        assert_eq!(dot_get(&json, "count").as_deref(), Some("42"));
    }

    #[test]
    fn extract_items_array_root() {
        let json: Value = serde_json::from_str(
            r#"[
                {"title": "A", "url": "https://a.com"},
                {"title": "B", "url": "https://b.com"}
            ]"#,
        )
        .unwrap();
        let def = CustomEngineDef {
            label: "T".into(),
            method: "GET".into(),
            url: "https://x.com".into(),
            query_param: None,
            body_template: None,
            headers: Default::default(),
            result_path: None,
            title_field: "title".into(),
            url_field: "url".into(),
            snippet_field: None,
            pre_request: None,
            limit_param: None,
        };
        let items = extract_items(&json, &def).unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "A");
        assert_eq!(items[1].url, "https://b.com");
    }

    #[test]
    fn extract_items_with_jsonpath() {
        let json: Value = serde_json::from_str(
            r#"{"data": {"items": [{"name": "X", "link": "https://x.com"}]}}"#,
        )
        .unwrap();
        let def = CustomEngineDef {
            label: "T".into(),
            method: "GET".into(),
            url: "https://x.com".into(),
            query_param: None,
            body_template: None,
            headers: Default::default(),
            result_path: Some("$.data.items[*]".into()),
            title_field: "name".into(),
            url_field: "link".into(),
            snippet_field: None,
            pre_request: None,
            limit_param: None,
        };
        let items = extract_items(&json, &def).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "X");
        assert_eq!(items[0].url, "https://x.com");
    }

    #[test]
    fn extract_items_nested_dot_fields() {
        let json: Value = serde_json::from_str(
            r#"[
                {
                    "repository": {"full_name": "rust-lang/rust", "html_url": "https://github.com/rust-lang/rust"},
                    "name": "main.rs"
                }
            ]"#,
        )
        .unwrap();
        let def = CustomEngineDef {
            label: "T".into(),
            method: "GET".into(),
            url: "https://x.com".into(),
            query_param: None,
            body_template: None,
            headers: Default::default(),
            result_path: None,
            title_field: "repository.full_name".into(),
            url_field: "repository.html_url".into(),
            snippet_field: Some("name".into()),
            pre_request: None,
            limit_param: None,
        };
        let items = extract_items(&json, &def).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "rust-lang/rust");
        assert_eq!(items[0].url, "https://github.com/rust-lang/rust");
        assert_eq!(items[0].snippet.as_deref(), Some("main.rs"));
    }
}

// ── Pre-request scripting (Boa JS engine) ──────────────────────────

#[cfg(feature = "pre-request-script")]
fn run_pre_request_script(
    mut req: reqwest::RequestBuilder,
    script: &str,
    query: &str,
    limit: usize,
    url: &str,
    method: &str,
) -> Result<reqwest::RequestBuilder> {
    use boa_engine::{Context, Source, property::Attribute};
    use boa_stdlib::load_default;

    let js_ctx = &mut Context::default();
    load_default(js_ctx).context("loading Boa stdlib")?;

    // Expose the `req` mutator object to JS.
    let req_obj = {
        let obj = boa_engine::object::Object::default();

        let url_js = boa_engine::JsValue::String(boa_engine::JsString::from(url));
        let _ = obj.set("url", url_js, false, js_ctx);

        let method_js = boa_engine::JsValue::String(boa_engine::JsString::from(method));
        let _ = obj.set("method", method_js, false, js_ctx);

        let query_js = boa_engine::JsValue::String(boa_engine::JsString::from(query));
        let _ = obj.set("query", query_js, false, js_ctx);

        let limit_js = boa_engine::JsValue::Integer(limit as i32);
        let _ = obj.set("limit", limit_js, false, js_ctx);

        // headers: an empty object the script can mutate.
        let headers_obj = boa_engine::object::Object::default();
        let _ = obj.set(
            "headers",
            boa_engine::JsValue::Object(headers_obj.clone()),
            false,
            js_ctx,
        );

        // body: mutable string.
        let body_js = boa_engine::JsValue::String(boa_engine::JsString::from(""));
        let _ = obj.set("body", body_js, false, js_ctx);

        obj
    };

    js_ctx
        .register_global_property("req", req_obj.clone(), Attribute::all())
        .context("registering req")?;

    js_ctx
        .eval(Source::from_bytes(script))
        .context("executing pre-request script")?;

    // Read back mutated values.
    let new_url = req_obj
        .get("url", js_ctx)
        .and_then(|v| v.as_string().map(|s| s.to_std_string_escaped()))
        .unwrap_or_else(|| url.to_string());

    let new_headers: Vec<(String, String)> = req_obj
        .get("headers", js_ctx)
        .and_then(|v| v.as_object().cloned())
        .map(|obj| {
            obj.properties()
                .map(|(k, v)| {
                    let val = v.as_string().map(|s| s.to_std_string_escaped()).unwrap_or_default();
                    (k.to_string(), val)
                })
                .collect()
        })
        .unwrap_or_default();

    let new_body = req_obj
        .get("body", js_ctx)
        .and_then(|v| v.as_string().map(|s| s.to_std_string_escaped()));

    // Rebuild the request.
    if new_url != url {
        req = match method {
            "GET" => reqwest::Client::new().get(&new_url),
            "POST" => reqwest::Client::new().post(&new_url),
            _ => return Err(anyhow!("unsupported method in pre-request: {method}")),
        };
    }

    for (k, v) in new_headers {
        req = req.header(&k, &v);
    }

    if let Some(b) = new_body {
        if !b.is_empty() && method == "POST" {
            req = req.body(b);
        }
    }

    Ok(req)
}
