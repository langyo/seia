# ブラウザモード

一部のエンジン —— Google、Baidu、Bing（API ではなく Web ページ）、Yandex —— は
非ブラウザのリクエストを強力にブロックします。seia はヘッドレスブラウザランタイムである
[tairitsu](https://github.com/celestia-island/tairitsu) を通じてこれらを駆動します。
seia は tairitsu の HTTP debug API と通信するため、ネイティブなブラウザバインディングは
**ありません**。

## tairitsu を動かす 2 つの方法

### 1. 外部デーモン（デフォルト）

tairitsu の debug サーバを別途起動し、seia からそこへ接続します。

```bash
# 端末 1 で
tairitsu debug --proxy http://localhost:7890

# 別の端末で
seia search "rust async" --engine google --browser --tairitsu http://127.0.0.1:3001
```

これにより、重いブラウザプロセスをアプリケーションのバイナリ外に置けます。

### 2. 組み込み（`embedded-browser` feature）

tairitsu の debug サーバを seia に*組み込んで*コンパイルします。別のデーモンは不要です。

```toml
[dependencies]
seia = { version = "0.1", features = ["embedded-browser"] }
```

```bash
seia search "rust async" --engine google --browser --embedded
```

`embedded` フラグはプロセス内サーバを起動します
（[`seia::embedded::start`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs) を参照）。

## ブラウザ検索の仕組み

ブラウザ検索は 3 つのステップで構成され、すべて tairitsu の HTTP API に対して発行されます。

1. **ナビゲート** —— エンジンの検索 URL へ `POST /navigate` を送信します。
2. **待機** —— 結果コンテナがレンダリングされるまで `POST /wait-for-selector` を送信します。
3. **抽出** —— `POST /evaluate` が JS のスニペットを実行し、DOM からタイトル・リンク・
   スニペットを読み出します。

各エンジンのセレクタと URL テンプレートは
[`SearchProfile`](https://github.com/celestia-island/seia/blob/dev/src/profiles.rs) に定義されています。

| プロファイル | 検索 URL | 結果コンテナ |
| --- | --- | --- |
| `google` | `google.com/search?q=` | `div.g` |
| `baidu` | `baidu.com/s?wd=` | `div.result, div.c-container` |
| `bing_web` | `bing.com/search?q=` | `li.b_algo` |
| `yandex` | `yandex.com/search/?text=` | `li.serps-item, div.Organic` |

## ブラウザクライアントを直接使う

```rust
use seia::{BrowserClient, profiles};

let client = BrowserClient::new("http://127.0.0.1:3001");

if !client.health().await.unwrap_or(false) {
    panic!("tairitsu browser not connected");
}

let profile = profiles::get_profile("google").unwrap();
let result = client.search("rust async", profile).await?;
```

CLI は内部的に `--engine <name> --browser` を対応するプロファイルへマッピングします
（一致するものがない場合は `google` プロファイルにフォールバックします）。
