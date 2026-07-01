<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>阅览互联网上的知识</strong>

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

[English](../en/README.md) · **简体中文** ·
[繁體中文](../zht/README.md) · [日本語](../ja/README.md) ·
[한국어](../ko/README.md) · [Français](../fr/README.md) ·
[Español](../es/README.md) · [Русский](../ru/README.md) ·
[العربية](../ar/README.md)

</div>

## 简介

seia 是一个多引擎 Web 搜索库与 CLI 工具。通过统一接口访问多样化的搜索后端，无需认证的引擎零配置即可使用。

## 快速开始

### CLI

```bash
# 基础搜索（无需 API 密钥）
seia search "rust 异步模式"

# 选择某个搜索引擎
seia search "克莱因瓶" --engine wikipedia

# JSON 输出
seia search "气候变化" --json

# 通过代理
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

### 作为库使用

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust 异步", Engine::Duckduckgo).await?;
```

## 开发

```bash
just ci          # 格式化检查 + clippy + 测试
just test        # 运行测试
just test-proxy  # 通过 localhost:7890 代理运行测试（见 tests/README）
```

## 支持的搜索引擎

所有引擎都经由各自官方的 HTTP API（在没有官方 API 时，则进行轻量 HTML 抓取）。seia 不内
置任何无头浏览器 —— 它是一个纯粹的 HTTP 客户端。

### 国际

| 引擎 | 官网 | 模式 | 认证 | 免费额度 | 状态 |
|------|------|------|------|---------|------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | 爬取 | 无 | 无限 | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | 无 | 无限 | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | 自建 | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000 / 月 | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000 / 月 | ✅ |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000 / 月 | ✅ |

### 国内（中国）

| 引擎 | 官网 | 模式 | 认证 | 状态 |
|------|------|------|------|------|
| 智谱（Zhipu） | [bigmodel.cn](https://bigmodel.cn) | API | `ZHIPU_API_KEY` | ✅ |
| 博查（Bocha） | [open.bochaai.com](https://open.bochaai.com) | API | `BOCHA_API_KEY` | ✅ |

> 智谱会经由若干底层引擎之一路由（智谱基础版/高阶版、搜狗、夸克）。用
> `ZHIPU_SEARCH_ENGINE` 环境变量选择其一（默认 `search_std`；也支持
> `search_pro`、`search_pro_sogou`、`search_pro_quark`）。

## 许可证

SySL-1.0（合成源码许可证）。详见 [LICENSE](../../LICENSE)。
