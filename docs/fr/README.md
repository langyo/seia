<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>La connaissance de toutes les sources</strong>

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

[English](../en/README.md) · [简体中文](../zhs/README.md) ·
[繁體中文](../zht/README.md) · [日本語](../ja/README.md) ·
[한국어](../ko/README.md) · **Français** ·
[Español](../es/README.md) · [Русский](../ru/README.md) ·
[العربية](../ar/README.md)

</div>

## Introduction

seia est une bibliothèque et un CLI de recherche web multi-moteurs. Il fournit une
interface unifiée pour interroger divers backends de recherche. Les moteurs qui ne
nécessitent pas d'authentification fonctionnent immédiatement sans configuration.

## Démarrage rapide

### CLI

```bash
# Recherche de base (sans clé API)
seia search "rust async patterns"

# Choisir un moteur spécifique
seia search "Klein bottle" --engine wikipedia

# Sortie JSON
seia search "climate change" --json

# Via un proxy
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

### Bibliothèque

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Duckduckgo).await?;
```

## Développement

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## Moteurs de recherche supportés

Tous les moteurs passent par leur API HTTP officielle (ou, lorsqu'aucune n'existe,
par un léger scraping HTML). Aucun navigateur headless n'est embarqué — seia est
un pur client HTTP.

### International

| Moteur | Site officiel | Mode | Authentification | Quota gratuit | État |
|--------|-------------|------|-----------------|-------------|------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | Scraping | aucun | illimité | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | aucun | illimité | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | auto-hébergé | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000/mois | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000/mois | ✅ |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000/mois | ✅ |

### National (Chine)

| Moteur | Site officiel | Mode | Authentification | État |
|--------|-------------|------|-----------------|------|
| 智谱 (Zhipu) | [bigmodel.cn](https://bigmodel.cn) | API | `ZHIPU_API_KEY` | ✅ |
| 博查 (Bocha) | [open.bochaai.com](https://open.bochaai.com) | API | `BOCHA_API_KEY` | ✅ |

> 智谱 route via l'un de plusieurs moteurs sous-jacents (智谱基础版/高阶版, 搜狗,
> 夸克). Choisissez-en un avec la variable d'environnement `ZHIPU_SEARCH_ENGINE`
> (`search_std` par défaut ; également `search_pro`, `search_pro_sogou`,
> `search_pro_quark`).

## Licence

SySL-1.0 (Synthetic Source License). Voir [LICENSE](../../LICENSE).
