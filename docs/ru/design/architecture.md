# Архитектура

seia — это единый крейт, поставляющий как библиотеку (`src/lib.rs`), так и CLI
(`src/main.rs`). Цель проектирования — **единая поверхность запроса, множество
бэкендов**: вызывающий выбирает `Engine` и получает один и тот же
`SearchResult` независимо от того, как именно был получен результат.

## Карта модулей

```
src/
├── lib.rs          публичная поверхность API + сервер embedded-browser
├── main.rs         clap CLI (search / engines)
├── engines.rs      перечисление Engine: as_str, api_key_env, needs_key, needs_browser
├── engines_impl/   по одному модулю на каждый API/парсинг-бэкенд
│   ├── duckduckgo.rs   парсинг (HTML)
│   ├── wikipedia.rs    API (JSON)
│   ├── tavily.rs       API (JSON, ключ)
│   └── searxng.rs      API (JSON, собственный хостинг)
├── client.rs       SearchClient + SearchOptions (путь API/парсинга)
├── browser.rs      BrowserClient (общается с tairitsu по HTTP)
├── profiles.rs     SearchProfile: CSS-селекторы + шаблон URL для каждого движка
├── extractor.rs    загрузчик полного содержимого страницы (для --fetch)
└── result.rs       SearchResult / SearchItem / SearchMode
```

## Три пути выполнения, один тип результата

Все три пути сходятся к
[`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs):

```
                        ┌─ engines_impl/* (API / парсинг) ─┐
query + Engine ─► SearchClient ─► унификация ─► SearchResult
                        └─ browser.rs (tairitsu HTTP) ─────┘
```

- **API** — `engines_impl::<engine>::search(&http, query, &opts)` вызывает
  провайдера и десериализует JSON в `SearchItem`.
- **Парсинг** — та же сигнатура, но разбирается HTML-страница результатов.
- **Браузер** — `BrowserClient::search` управляет tairitsu; `SearchProfile`
  движка предоставляет URL и CSS-селекторы, используемые внедряемым JS
  извлечения.

`SearchMode` (`Api` / `Scrape` / `Browser`) фиксирует, каким путём получен
результат, поэтому вызывающий может отличить, например, кешированный ответ API
от отрендеренной страницы.

## Диспетчеризация

`SearchClient::search_with_options` — это плоский `match` по `Engine`. Добавить
бэкенд означает: реализовать одну функцию в `engines_impl/`, добавить вариант в
`Engine` и добавить ветвь `match`. Никаких trait-объектов или динамической
диспетчеризации — набор движков замкнут и известен на этапе компиляции, что
сохраняет API предсказуемым, а бинарник — компактным.

## Обогащение содержимым

`SearchOptions::fetch_content` — ортогональная забота: после того как движок
вернёт `SearchItem`, `extractor::fetch_content` загружает и очищает каждую
страницу. Это не зависит от движка и работает для любого режима.

## Граница интеграции с браузером

`tairitsu-packager` — **необязательная** зависимость, защищённая функцией
`embedded-browser`. Без неё seia не содержит браузерного кода и подключается к
внешнему демону tairitsu по обычному HTTP (`BrowserClient`). С ней
`seia::embedded::start` запускает debug-сервер внутри процесса. Так сборка по
умолчанию остаётся лёгкой, а публикуемый крейт — свободным от тяжёлых браузерных
зависимостей.
