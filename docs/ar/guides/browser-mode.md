# وضع المتصفح

بعض المحركات — Google وBaidu وBing (صفحة الويب لا الـ API) وYandex — تحجب بشراسة
الطلبات غير الصادرة من متصفح. يقودها seia عبر
[tairitsu](https://github.com/celestia-island/tairitsu)، وهو زمن تشغيل متصفح بلا
واجهة. يتحدّث seia مع HTTP debug API الخاص بـ tairitsu، لذا **لا توجد** أي روابط
(bindings) متصفح أصلية.

## طريقتان لتشغيل tairitsu

### 1. خادم خفيّ خارجي (افتراضي)

شغّل خادم tairitsu للتنقيح (debug) منفصلًا ووجّه seia إليه:

```bash
# في طرفية
tairitsu debug --proxy http://localhost:7890

# في طرفية أخرى
seia search "rust async" --engine google --browser --tairitsu http://127.0.0.1:3001
```

هذا يُبقي عملية المتصفح الثقيلة خارج الملف الثنائي لتطبيقك.

### 2. مضمَّن (ميزة `embedded-browser`)

جمّع خادم tairitsu للتنقيح *داخل* seia. لا حاجة لخادم خفيّ منفصل:

```toml
[dependencies]
seia = { version = "0.1", features = ["embedded-browser"] }
```

```bash
seia search "rust async" --engine google --browser --embedded
```

العلامة `embedded` تُطلق الخادم داخل العملية (انظر
[`seia::embedded::start`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs)).

## كيف يعمل بحث المتصفح

كل بحث عبر المتصفح ثلاث خطوات، تُرسل كلها إلى HTTP API الخاص بـ tairitsu:

1. **التنقّل** — `POST /navigate` إلى عنوان بحث المحرك.
2. **الانتظار** — `POST /wait-for-selector` حتى يُعرض حاوي النتائج.
3. **الاستخراج** — `POST /evaluate` يشغّل مقطع JS يقرأ العناوين والروابط والملخّصات
   من DOM.

تقع محدّدات (selectors) وعنوان URL لكل محرك في
[`SearchProfile`](https://github.com/celestia-island/seia/blob/dev/src/profiles.rs):

| الملف (Profile) | عنوان البحث | حاوي النتائج |
| --- | --- | --- |
| `google` | `google.com/search?q=` | `div.g` |
| `baidu` | `baidu.com/s?wd=` | `div.result, div.c-container` |
| `bing_web` | `bing.com/search?q=` | `li.b_algo` |
| `yandex` | `yandex.com/search/?text=` | `li.serps-item, div.Organic` |

## استخدام عميل المتصفح مباشرةً

```rust
use seia::{BrowserClient, profiles};

let client = BrowserClient::new("http://127.0.0.1:3001");

if !client.health().await.unwrap_or(false) {
    panic!("tairitsu browser not connected");
}

let profile = profiles::get_profile("google").unwrap();
let result = client.search("rust async", profile).await?;
```

يُعيد CLI تعيين `--engine <name> --browser` إلى الـ profile المطابق داخليًا
(مع التراجع إلى الـ profile `google` عند عدم وجود تطابق).
