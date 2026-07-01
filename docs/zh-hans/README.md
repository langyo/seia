# seia — 通用搜索引擎抽象层

<div align="center">

**Rust 通用搜索引擎抽象库**

多后端 Web 搜索库和 CLI — 支持 API 模式、爬取模式、无头浏览器模式。

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![CI](https://github.com/celestia-island/seia/actions/workflows/checks.yml/badge.svg)](https://github.com/celestia-island/seia/actions)

</div>

## 简介

seia 是一个通用的搜索引擎抽象层。它提供统一的接口来访问多个搜索后端——从免费的 HTML 爬取到付费 API——无需在切换引擎时修改代码。

## 快速开始

```bash
# 安装
cargo install --path .

# 基本搜索
seia search "rust 异步模式"

# 选择引擎
seia search "Klein bottle" --engine wikipedia

# JSON 输出
seia search "气候变化" --engine duckduckgo --json
```

## 引擎列表

| 引擎 | 模式 | 认证 | 免费额度 | 申请方式 |
|------|------|------|---------|---------|
| DuckDuckGo | 爬取 | 无 | 无限 | N/A |
| Wikipedia | API | 无 | 无限 | N/A |
| SearXNG | API | 无 | 自建 | [部署文档](https://searxng.github.io/searxng/admin/installation.html) |
| Tavily | API | `TAVILY_API_KEY` | 1000/月 | [app.tavily.com](https://app.tavily.com) |
| Bing | API | `BING_SEARCH_API_KEY` | 1000/月 | [Azure Portal](https://portal.azure.com) |
| Brave | API | `BRAVE_SEARCH_API_KEY` | 2000/月 | [brave.com/search/api](https://brave.com/search/api/) |

## 代理支持

```bash
export HTTPS_PROXY=http://localhost:7890
seia search "hello world"
```

## 许可证

SySL-1.0（合成源码许可证）。
