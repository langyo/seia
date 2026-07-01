<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>Знания из любых источников</strong>

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

[English](../en/README.md) · [简体中文](../zhs/README.md) ·
[繁體中文](../zht/README.md) · [日本語](../ja/README.md) ·
[한국어](../ko/README.md) · [Français](../fr/README.md) ·
[Español](../es/README.md) · **Русский** ·
[العربية](../ar/README.md)

</div>

## Введение

seia — это мультидвижковая библиотека и CLI для веб-поиска. Она предоставляет единый
интерфейс для запросов к различным поисковым бэкендам. Движки, не требующие
аутентификации, работают сразу без какой-либо настройки.

## Быстрый старт

### CLI

```bash
# Базовый поиск (без API-ключа)
seia search "rust async patterns"

# Выбор конкретного движка
seia search "Klein bottle" --engine wikipedia

# Вывод в JSON
seia search "climate change" --json

# Через прокси
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

### Библиотека

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Duckduckgo).await?;
```

## Разработка

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## Поддерживаемые поисковые движки

Все движки обращаются к своему официальному HTTP-API (или, при отсутствии API,
к лёгкому парсингу HTML). Никакого headless-браузера не прилагается — seia —
это чистый HTTP-клиент.

### Международные

| Движок | Официальный сайт | Режим | Аутентификация | Бесплатный лимит | Статус |
|--------|-----------------|------|---------------|-----------------|--------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | Парсинг | нет | безлимитный | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | нет | безлимитный | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | свой хостинг | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000/мес | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000/мес | ✅ |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000/мес | ✅ |

### Национальные (Китай)

| Движок | Официальный сайт | Режим | Аутентификация | Статус |
|--------|-----------------|------|---------------|--------|
| 智谱 (Zhipu) | [bigmodel.cn](https://bigmodel.cn) | API | `ZHIPU_API_KEY` | ✅ |
| 博查 (Bocha) | [open.bochaai.com](https://open.bochaai.com) | API | `BOCHA_API_KEY` | ✅ |

> 智谱 маршрутизируется через один из нескольких нижележащих движков (智谱基础版/高阶版,
> 搜狗, 夸克). Выберите нужный через переменную окружения `ZHIPU_SEARCH_ENGINE`
> (`search_std` по умолчанию; также `search_pro`, `search_pro_sogou`,
> `search_pro_quark`).

## Лицензия

SySL-1.0 (Synthetic Source License). См. [LICENSE](../../LICENSE).
