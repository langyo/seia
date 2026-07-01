<p align="center"><img src="../logo.webp" alt="seia" width="240" /></p>

<h1 align="center">seia</h1>

<div align="center">

<strong>المعرفة من كل المصادر</strong>

[![License: SySL-1.0](https://img.shields.io/badge/License-SySL--1.0-blue.svg)](../../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/seia)](https://docs.rs/seia)
[![CI](https://img.shields.io/github/actions/workflow/status/celestia-island/seia/checks.yml)](https://github.com/celestia-island/seia/actions/workflows/checks.yml)
[![Docs](https://img.shields.io/badge/docs-seia.docs.celestia.world-blue)](https://seia.docs.celestia.world)

[English](../en/README.md) · [简体中文](../zhs/README.md) ·
[繁體中文](../zht/README.md) · [日本語](../ja/README.md) ·
[한국어](../ko/README.md) · [Français](../fr/README.md) ·
[Español](../es/README.md) · [Русский](../ru/README.md) ·
**العربية**

</div>

## مقدمة

seia هي مكتبة وأداة سطر أوامر للبحث متعدد المحركات. توفر واجهة موحدة للاستعلام
عن خلفيات بحث متنوعة. المحركات التي لا تتطلب مصادقة تعمل فورًا دون أي إعداد.

## البداية السريعة

### CLI

```bash
# بحث أساسي (بلا مفتاح API)
seia search "rust async patterns"

# اختيار محرك معين
seia search "Klein bottle" --engine wikipedia

# مخرجات JSON
seia search "climate change" --json

# عبر وكيل (proxy)
HTTPS_PROXY=http://localhost:7890 seia search "hello world"
```

### المكتبة

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::Duckduckgo).await?;
```

## التطوير

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## المحركات المدعومة

تمرّ جميع المحركات عبر واجهة HTTP API الرسمية الخاصة بها (أو، حيث لا توجد واجهة، عبر
كشط HTML خفيف). لا يُضمَّن أي متصفح بلا واجهة — seia هو عميل HTTP نقي.

### دولي

| المحرك | الموقع الرسمي | الوضع | المصادقة | الحد المجاني | الحالة |
|--------|-------------|------|---------|-----------|--------|
| DuckDuckGo | [duckduckgo.com](https://duckduckgo.com) | كشط | بدون | غير محدود | ✅ |
| Wikipedia | [wikipedia.org](https://www.wikipedia.org) | API | بدون | غير محدود | ✅ |
| SearXNG | [searxng.org](https://searxng.org) | API | `SEARXNG_URL` | استضافة ذاتية | ✅ |
| Tavily | [tavily.com](https://tavily.com) | API | `TAVILY_API_KEY` | 1 000/شهر | ✅ |
| Bing | [bing.com](https://www.bing.com) | API | `BING_SEARCH_API_KEY` | 1 000/شهر | ✅ |
| Brave | [brave.com/search](https://brave.com/search) | API | `BRAVE_SEARCH_API_KEY` | 2 000/شهر | ✅ |

### محلي (الصين)

| المحرك | الموقع الرسمي | الوضع | المصادقة | الحالة |
|--------|-------------|------|---------|--------|
| 智谱 (Zhipu) | [bigmodel.cn](https://bigmodel.cn) | API | `ZHIPU_API_KEY` | ✅ |
| 博查 (Bocha) | [open.bochaai.com](https://open.bochaai.com) | API | `BOCHA_API_KEY` | ✅ |

> يوجّه 智谱 الطلب عبر إحدى عدّة محركات خلفية (智谱基础版/高阶版، 搜狗، 夸克). اختر
> إحداها بمتغيّر البيئة `ZHIPU_SEARCH_ENGINE` (`search_std` افتراضيًا؛ وكذلك
> `search_pro`، `search_pro_sogou`، `search_pro_quark`).

## الترخيص

SySL-1.0 (ترخيص المصدر التركيبي). انظر [LICENSE](../../LICENSE).
