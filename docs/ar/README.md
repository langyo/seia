# seia

**استعلام واحد، كل محركات البحث.**

بحث ويب متعدد المحركات لِـ Rust. تعمل المحركات المجانية مباشرةً دون أي إعداد.

## مقدمة

يتيح لك seia البحث في الويب عبر DuckDuckGo وTavily وWikipedia وSearXNG وBing وBrave
وGoogle وBaidu وغيرها — كلها خلف واجهة واحدة. تعمل المحركات المجانية مباشرةً دون أي
إعداد.

## البداية السريعة

### CLI

```bash
# بحث أساسي (DuckDuckGo، مجاني، بلا مفتاح)
seia search "rust async patterns"

# Wikipedia (مجاني، أكاديمي)
seia search "Klein bottle" --engine wikipedia

# مخرجات JSON
seia search "climate change" --json

# عبر وكيل (proxy)
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# وضع المتصفح (Google/Baidu عبر tairitsu)
seia search "query" --engine google --browser
```

### المكتبة

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::DuckDuckGo).await?;
```

## المحركات

| المحرك | الوضع | المصادقة | الحالة |
|--------|------|------|--------|
| DuckDuckGo | كشط | بدون | ✅ |
| Wikipedia | API | بدون | ✅ |
| SearXNG | API | `SEARXNG_URL` | ✅ |
| Tavily | API | `TAVILY_API_KEY` | ✅ |
| Bing | API | `BING_SEARCH_API_KEY` | 🔲 |
| Brave | API | `BRAVE_SEARCH_API_KEY` | 🔲 |
| Google | متصفح | tairitsu | ✅ |
| Baidu | متصفح | tairitsu | ✅ |
| Bing Web | متصفح | tairitsu | ✅ |
| Yandex | متصفح | tairitsu | ✅ |

تستخدم محركات وضع المتصفح [tairitsu](https://github.com/celestia-island/tairitsu)
للعرض بلا واجهة. إمّا شغّل خادمًا خفيًّا مستقلًا، أو فعّل ميزة `embedded-browser`
لتجميع tairitsu داخل العملية.

## التطوير

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## الترخيص

SySL-1.0 (ترخيص المصدر التركيبي). انظر [LICENSE](../../LICENSE).
