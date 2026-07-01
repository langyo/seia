<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>Conocimiento de todas las fuentes</strong>

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

[English](../en/README.md) · [简体中文](../zhs/README.md) ·
[繁體中文](../zht/README.md) · [日本語](../ja/README.md) ·
[한국어](../ko/README.md) · [Français](../fr/README.md) ·
**Español** · [Русский](../ru/README.md) ·
[العربية](../ar/README.md)

</div>

## Introducción

seia es una biblioteca y CLI de búsqueda web multimotor. Proporciona una interfaz
unificada para consultar diversos backends de búsqueda. Los motores que no requieren
autenticación funcionan de inmediato sin configuración.

## Inicio rápido

### CLI

```bash
# Búsqueda básica (sin clave API)
seia search "rust async patterns"

# Elegir un motor específico
seia search "Klein bottle" --engine wikipedia

# Salida JSON
seia search "climate change" --json

# A través de un proxy
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

### Biblioteca

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Duckduckgo).await?;
```

## Desarrollo

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## Motores soportados

Todos los motores pasan por su API HTTP oficial (o, donde no existe, por un raspado
ligero del HTML). No se incluye ningún navegador sin interfaz: seia es un cliente
HTTP puro.

### Internacional

| Motor | Sitio oficial | Modo | Autenticación | Cuota gratuita | Estado |
|-------|-------------|------|---------------|---------------|--------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | Raspado | Ninguno | ilimitado | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | Ninguno | ilimitado | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | autoalojado | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000/mes | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000/mes | ✅ |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000/mes | ✅ |

### Nacional (China)

| Motor | Sitio oficial | Modo | Autenticación | Estado |
|-------|-------------|------|---------------|--------|
| 智谱 (Zhipu) | [bigmodel.cn](https://bigmodel.cn) | API | `ZHIPU_API_KEY` | ✅ |
| 博查 (Bocha) | [open.bochaai.com](https://open.bochaai.com) | API | `BOCHA_API_KEY` | ✅ |

> 智谱 enruta a través de uno de varios motores de respaldo (智谱基础版/高阶版, 搜狗,
> 夸克). Elige uno con la variable de entorno `ZHIPU_SEARCH_ENGINE`
> (`search_std` por defecto; también `search_pro`, `search_pro_sogou`,
> `search_pro_quark`).

## Licencia

SySL-1.0 (Synthetic Source License). Consulta [LICENSE](../../LICENSE).
