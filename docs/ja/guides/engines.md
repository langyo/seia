# エンジン

seia はすべてのバックエンドを単一の [`Engine`](https://github.com/celestia-island/seia/blob/dev/src/engines.rs)
列挙型で公開しているため、バックエンドを切り替えてもクエリコードは一切変更されません。

## 3 つの実行モード

| モード | 仕組み | エンジン |
| --- | --- | --- |
| **API** | 検索プロバイダの HTTP API を呼び出して JSON を解析します。 | Tavily、SearXNG、Wikipedia |
| **スクレイプ** | HTML の検索結果ページを取得し、ヒットを抽出します。 | DuckDuckGo |
| **ブラウザ** | ヘッドレスブラウザ（[tairitsu](https://github.com/celestia-island/tairitsu) 経由）を駆動して JS 多めのページをレンダリングします。 | Google、Baidu、Bing（Web）、Yandex |

API とスクレイプのモードは HTTP クライアントだけで動作します。ブラウザモードについては
[ブラウザモード](./browser-mode.md) を参照してください。

## エンジン一覧

| エンジン | 列挙値 | モード | 認証 | 無料枠 |
| --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | スクレイプ | なし | 無制限 |
| Wikipedia | `Wikipedia` | API | なし | 無制限 |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | 自己ホスト |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000 / 月 |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000 / 月 |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000 / 月 |
| Google | ブラウザプロファイル | ブラウザ | tairitsu | — |
| Baidu | ブラウザプロファイル | ブラウザ | tairitsu | — |
| Bing（Web） | ブラウザプロファイル | ブラウザ | tairitsu | — |
| Yandex | ブラウザプロファイル | ブラウザ | tairitsu | — |

> Bing と Brave の API バックエンドはスタブ実装です（`Engine::Bing` / `Engine::Brave`
> は "not yet implemented" エラーを返します）。ブラウザプロファイルを使うか、実装を
> [コントリビート](https://github.com/celestia-island/seia)してください。

## エンジンの選択

CLI:

```bash
seia search "query" --engine wikipedia
```

ライブラリ:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;
```

## エンジンのメタデータを確認する

`Engine` は自身のメタデータを保持しているため、ハードコードせずに UI を構築できます。

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing
    println!("  needs key? {}", engine.needs_key());
    println!("  key env:    {:?}", engine.api_key_env());
}
```
