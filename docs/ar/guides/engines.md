# المحركات

يعرّض seia كل خلفية عبر الـ enum الوحيد
[`Engine`](https://github.com/celestia-island/seia/blob/dev/src/engines.rs)، لذا فإن
تبديل الخلفية لا يمسّ كود الاستعلام أبدًا.

## ثلاثة أوضاع تنفيذ

| الوضع | طريقة العمل | المحركات |
| --- | --- | --- |
| **API** | يستدعي HTTP API لمزوّد البحث ويحلّل JSON. | Tavily وSearXNG وWikipedia |
| **كشط** | يجلب صفحة نتائج HTML ويستخرج النتائج. | DuckDuckGo |
| **متصفح** | يقود متصفحًا بلا واجهة (عبر [tairitsu](https://github.com/celestia-island/tairitsu)) لعرض الصفحات الغنية بـ JS. | Google وBaidu وBing (ويب) وYandex |

وضعا API والكشط لا يحتاجان سوى عميل HTTP. أمّا وضع المتصفح فيُشرح في
[وضع المتصفح](./browser-mode.md).

## مصفوفة المحركات

| المحرك | قيمة الـ Enum | الوضع | المصادقة | الباقة المجانية |
| --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | كشط | بدون | غير محدود |
| Wikipedia | `Wikipedia` | API | بدون | غير محدود |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | مستضاف ذاتيًا |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000 / شهر |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000 / شهر |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000 / شهر |
| Google | browser profile | متصفح | tairitsu | — |
| Baidu | browser profile | متصفح | tairitsu | — |
| Bing (ويب) | browser profile | متصفح | tairitsu | — |
| Yandex | browser profile | متصفح | tairitsu | — |

> خلفيتا API لِكلٍّ من Bing وBrave هما مجرّد هياكل مؤقّة (`Engine::Bing` / `Engine::Brave`
> تُرجع خطأ "not yet implemented"). استخدم browser profile أو
> [ساهم](https://github.com/celestia-island/seia) بتنفيذ.

## اختيار محرك

CLI:

```bash
seia search "query" --engine wikipedia
```

المكتبة:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;
```

## فحص البيانات الوصفية للمحرك

يحمل `Engine` بياناته الوصفية الخاصة، لذا يمكنك بناء واجهات دون ترميز ثابت (hard-coding):

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing
    println!("  يتطلّب مفتاحًا؟ {}", engine.needs_key());
    println!("  متغيّر المفتاح:    {:?}", engine.api_key_env());
}
```
