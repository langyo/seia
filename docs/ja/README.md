# seia

**ひとつのクエリで、すべての検索エンジンへ。**

Rust 向けマルチエンジン Web 検索。無料エンジンがすぐに使えます。

## はじめに

seia は DuckDuckGo、Tavily、Wikipedia、SearXNG、Bing、Brave、Google、Baidu
など多数のバックエンドを、ひとつのインターフェースで扱えるようにします。無料エンジンは
設定なしですぐに使えます。

## クイックスタート

### CLI

```bash
# 基本検索（DuckDuckGo、無料、キー不要）
seia search "rust async patterns"

# Wikipedia（無料、学術）
seia search "Klein bottle" --engine wikipedia

# JSON 出力
seia search "climate change" --json

# プロキシ経由
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# ブラウザモード（Google/Baidu を tairitsu 経由で）
seia search "query" --engine google --browser
```

### ライブラリ

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::DuckDuckGo).await?;
```

## エンジン

| エンジン | モード | 認証 | 状態 |
|--------|------|------|--------|
| DuckDuckGo | スクレイプ | なし | ✅ |
| Wikipedia | API | なし | ✅ |
| SearXNG | API | `SEARXNG_URL` | ✅ |
| Tavily | API | `TAVILY_API_KEY` | ✅ |
| Bing | API | `BING_SEARCH_API_KEY` | 🔲 |
| Brave | API | `BRAVE_SEARCH_API_KEY` | 🔲 |
| Google | ブラウザ | tairitsu | ✅ |
| Baidu | ブラウザ | tairitsu | ✅ |
| Bing Web | ブラウザ | tairitsu | ✅ |
| Yandex | ブラウザ | tairitsu | ✅ |

ブラウザモードのエンジンはヘッドレスレンダリングに [tairitsu](https://github.com/celestia-island/tairitsu)
を使用します。スタンドアロンのデーモンを動かすか、`embedded-browser` feature を有効にして
tairitsu をプロセス内に組み込んでください。

## 開発

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## ライセンス

SySL-1.0（Synthetic Source License）。詳しくは [LICENSE](../../LICENSE) を参照してください。
