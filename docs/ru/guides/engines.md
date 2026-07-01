# Поисковые движки

seia предоставляет каждый бэкенд через единое перечисление
[`Engine`](https://github.com/celestia-island/seia/blob/dev/src/engines.rs),
поэтому переключение бэкендов никогда не затрагивает ваш код запроса.

## Три режима выполнения

| Режим | Как работает | Движки |
| --- | --- | --- |
| **API** | Вызывает HTTP-API поискового провайдера и разбирает JSON. | Tavily, SearXNG, Wikipedia |
| **Парсинг** | Загружает HTML-страницу результатов и извлекает совпадения. | DuckDuckGo |
| **Браузер** | Управляет headless-браузером (через [tairitsu](https://github.com/celestia-island/tairitsu)) для рендеринга страниц, насыщенных JS. | Google, Baidu, Bing (веб), Yandex |

Для режимов API и парсинга достаточно только HTTP-клиента. Режим браузера
описан в [Режим браузера](./browser-mode.md).

## Матрица движков

| Движок | Значение enum | Режим | Аутентификация | Бесплатный лимит |
| --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | Парсинг | нет | без ограничений |
| Wikipedia | `Wikipedia` | API | нет | без ограничений |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | собственный хостинг |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000 / месяц |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000 / месяц |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000 / месяц |
| Google | профиль браузера | Браузер | tairitsu | — |
| Baidu | профиль браузера | Браузер | tairitsu | — |
| Bing (веб) | профиль браузера | Браузер | tairitsu | — |
| Yandex | профиль браузера | Браузер | tairitsu | — |

> Бэкенды API для Bing и Brave пока заглушены (`Engine::Bing` / `Engine::Brave`
> возвращают ошибку «not yet implemented»). Используйте профили браузера или
> [внесите](https://github.com/celestia-island/seia) реализацию.

## Выбор движка

CLI:

```bash
seia search "query" --engine wikipedia
```

Библиотека:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;
```

## Просмотр метаданных движка

`Engine` содержит собственные метаданные, поэтому интерфейсы можно строить
без жёстко заданных значений:

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing
    println!("  нужен ключ? {}", engine.needs_key());
    println!("  env ключа:  {:?}", engine.api_key_env());
}
```
