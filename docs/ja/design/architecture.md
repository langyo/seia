# アーキテクチャ

seia はライブラリ（`src/lib.rs`）と CLI（`src/main.rs`）を兼ね備えた単一の crate です。
設計目標は**ひとつのクエリインターフェース、複数のバックエンド**です。呼び出し側は
`Engine` を選ぶだけで、結果の取得方法によらず同じ `SearchResult` を得られます。

## モジュール構成

```
src/
├── lib.rs          公開 API 群 + embedded-browser サーバ
├── main.rs         clap CLI（search / engines）
├── engines.rs      Engine 列挙型: as_str、api_key_env、needs_key、needs_browser
├── engines_impl/   API/スクレイプ バックエンドごとに 1 モジュール
│   ├── duckduckgo.rs   スクレイプ（HTML）
│   ├── wikipedia.rs    API（JSON）
│   ├── tavily.rs       API（JSON、キー必要）
│   └── searxng.rs      API（JSON、自己ホスト）
├── client.rs       SearchClient + SearchOptions（API/スクレイプのパス）
├── browser.rs      BrowserClient（HTTP で tairitsu と通信）
├── profiles.rs     SearchProfile: エンジンごとの CSS セレクタ + URL テンプレート
├── extractor.rs    ページ全文取得（--fetch 用）
└── result.rs       SearchResult / SearchItem / SearchMode
```

## 3 つの実行パス、1 つの結果型

3 つのパスはいずれも
[`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs) に集約されます。

```
                       ┌─ engines_impl/*（API / スクレイプ）─┐
query + Engine ─► SearchClient ─► 統一 ─► SearchResult
                       └─ browser.rs（tairitsu HTTP）──────┘
```

- **API** —— `engines_impl::<engine>::search(&http, query, &opts)` がプロバイダを呼び出し、
  JSON を `SearchItem` にデシリアライズします。
- **スクレイプ** —— 同じシグネチャですが、HTML の結果ページを解析します。
- **ブラウザ** —— `BrowserClient::search` が tairitsu を駆動します。エンジンごとの
  `SearchProfile` が URL と、注入される抽出 JS が使う CSS セレクタを提供します。

`SearchMode`（`Api` / `Scrape` / `Browser`）はどのパスが結果を生成したかを記録するため、
呼び出し側は例えばキャッシュされた API の応答とレンダリングされたページを区別できます。

## ディスパッチ

`SearchClient::search_with_options` は `Engine` に対するフラットな `match` です。
バックエンドを追加するには、`engines_impl/` に 1 つの関数を実装し、`Engine` のバリアントを
追加し、`match` の腕を 1 つ増やします。トレイトオブジェクトや動的ディスパッチはありません
—— エンジンの集合は閉じておりコンパイル時に確定しているため、API は予測可能でバイナリも
小さくなります。

## 本文の補完

`SearchOptions::fetch_content` は直交する関心事です。エンジンが `SearchItem` を返した後、
`extractor::fetch_content` が各ページをダウンロードして整形します。これはエンジンに依存せず、
どのモードでも機能します。

## ブラウザ統合の境界

`tairitsu-packager` は `embedded-browser` feature でゲートされた**オプション**の依存関係です。
無効の場合、seia にはブラウザのコードが一切含まれず、通常の HTTP で外部の tairitsu
デーモンに接続します（`BrowserClient`）。有効の場合、`seia::embedded::start` がプロセス内で
debug サーバを起動します。これにより、デフォルトのビルドは軽量に保たれ、公開する crate は
重いブラウザ依存を背負いません。
