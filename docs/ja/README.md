<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>あらゆる情報源から知識を</strong>

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

[English](../en/README.md) · [简体中文](../zhs/README.md) ·
[繁體中文](../zht/README.md) · **日本語** ·
[한국어](../ko/README.md) · [Français](../fr/README.md) ·
[Español](../es/README.md) · [Русский](../ru/README.md) ·
[العربية](../ar/README.md)

</div>

## はじめに

seia はマルチエンジン Web 検索ライブラリ兼 CLI です。多様な検索バックエンドを
単一のインターフェースで利用できます。認証不要のエンジンは設定なしですぐに使えます。

## クイックスタート

### CLI

```bash
# 基本検索（API キー不要）
seia search "rust async patterns"

# 特定のエンジンを選択
seia search "Klein bottle" --engine wikipedia

# JSON 出力
seia search "climate change" --json

# プロキシ経由
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

### ライブラリ

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Duckduckgo).await?;
```

## 開発

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## 対応検索エンジン

すべてのエンジンは公式の HTTP API（API がない場合は軽量な HTML スクレイプ）経由でアクセス
します。ヘッドレスブラウザは一切バンドルされておらず、seia は純粋な HTTP クライアントです。

### 国際

| エンジン | 公式サイト | モード | 認証 | 無料枠 | 状態 |
|--------|--------|------|------|------|--------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | スクレイプ | なし | 無制限 | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | なし | 無制限 | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | セルフホスト | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000/月 | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000/月 | ✅ |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000/月 | ✅ |

### 国内（中国）

| エンジン | 公式サイト | モード | 認証 | 状態 |
|--------|---------|------|------|--------|
| 智谱 (Zhipu) | [bigmodel.cn](https://bigmodel.cn) | API | `ZHIPU_API_KEY` | ✅ |
| 博查 (Bocha) | [open.bochaai.com](https://open.bochaai.com) | API | `BOCHA_API_KEY` | ✅ |

> 智谱は複数のバッキングエンジン（智谱基础版/高阶版、搜狗、夸克）のいずれかを経由します。
> `ZHIPU_SEARCH_ENGINE` 環境変数で選択してください（デフォルトは `search_std`。
> `search_pro`、`search_pro_sogou`、`search_pro_quark` も指定可能）。

## ライセンス

SySL-1.0（Synthetic Source License）。詳しくは [LICENSE](../../LICENSE) を参照してください。
