# البنية

seia هو crate واحد يوفّر كلًّا من مكتبة (`src/lib.rs`) وCLI (`src/main.rs`). الهدف
التصميمي هو **سطح استعلام واحد، خلفيات عديدة**: يختار المُستدعي `Engine` ويحصل على نفس
`SearchResult` بصرف النظر عن كيفية الحصول على النتيجة.

## خريطة الوحدات

```
src/
├── lib.rs          سطح الـ API العام + خادم embedded-browser
├── main.rs         clap CLI (search / engines)
├── engines.rs      Engine enum: as_str وapi_key_env وneeds_key وneeds_browser
├── engines_impl/   وحدة لكل خلفية API/كشط
│   ├── duckduckgo.rs   كشط (HTML)
│   ├── wikipedia.rs    API (JSON)
│   ├── tavily.rs       API (JSON، بمفتاح)
│   └── searxng.rs      API (JSON، مستضاف ذاتيًا)
├── client.rs       SearchClient + SearchOptions (مسار API/كشط)
├── browser.rs      BrowserClient (يتواصل مع tairitsu عبر HTTP)
├── profiles.rs     SearchProfile: محدّدات CSS لكل محرك + قالب URL
├── extractor.rs    جالب محتوى الصفحة الكامل (لِـ --fetch)
└── result.rs       SearchResult / SearchItem / SearchMode
```

## ثلاثة مسارات تنفيذ، نوع نتيجة واحد

تتقارب المسارات الثلاثة كلها على
[`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs):

```
                       ┌─ engines_impl/* (API / كشط) ─┐
query + Engine ─► SearchClient ─► توحيد ─► SearchResult
                       └─ browser.rs (tairitsu HTTP) ────┘
```

- **API** — يستدعي `engines_impl::<engine>::search(&http, query, &opts)` المزوّد،
  ويُلغي تسلسل JSON إلى عناصر `SearchItem`.
- **كشط** — نفس التوقيع (signature)، لكنه يحلّل صفحة نتائج HTML.
- **متصفح** — يقود `BrowserClient::search` محرّك tairitsu؛ ويوفّر `SearchProfile`
  الخاص بكل محرك عنوان URL ومحدّدات CSS التي يستخدمها JS المُحقن للاستخراج.

يسجّل `SearchMode` (`Api` / `Scrape` / `Browser`) أيُّ مسار أنتج النتيجة، فيتمكّن
المُستدعي من التمييز، مثلًا، بين إجابة API مخزّنة وصفحة معروضة.

## الإرسال (Dispatch)

`SearchClient::search_with_options` هو `match` مسطّح على `Engine`. وإضافة خلفية تعني:
تنفيذ دالة واحدة في `engines_impl/`، وإضافة مُتغيّر `Engine`، وإضافة ذراع `match`. لا
يوجد trait object أو إرسال ديناميكي — مجموعة المحركات مغلقة ومعروفة وقت التجميع، مما
يجعل الـ API متوقّعًا والملف الثنائي صغيرًا.

## إثراء المحتوى

`SearchOptions::fetch_content` شأنٌ متعامد (orthogonal): بعد أن تُرجع المحرّكات
عناصر `SearchItem`، تنزّل `extractor::fetch_content` كل صفحة وتنظّفها. هذا مستقل عن
المحرّك ويعمل مع أي وضع.

## حدّ تكامل المتصفح

`tairitsu-packager` اعتماديّة **اختيارية**، محجوبة خلف ميزة `embedded-browser`. بدونها
لا يحتوي seia على أي كود متصفّح ويتّصل بخادم tairitsu خارجي عبر HTTP عادي
(`BrowserClient`). ومعها يُطلق `seia::embedded::start` خادم التنقيح داخل العملية. هذا
يُبقي البناء الافتراضي خفيفًا ويُخلي الـ crate القابل للنشر من الاعتماديّات الثقيلة
الخاصة بالمتصفّح.
