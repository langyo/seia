# Режим браузера

Некоторые движки — Google, Baidu, Bing (веб-страница, а не API), Yandex —
агрессивно блокируют запросы не из браузера. seia работает с ними через
[tairitsu](https://github.com/celestia-island/tairitsu) — среду выполнения
headless-браузера. seia обращается к HTTP debug-API tairitsu, поэтому
собственных **привязок** к браузеру **нет**.

## Два способа запустить tairitsu

### 1. Внешний демон (по умолчанию)

Запустите debug-сервер tairitsu отдельно и укажите seia на него:

```bash
# в одном терминале
tairitsu debug --proxy http://localhost:7890

# в другом
seia search "rust async" --engine google --browser --tairitsu http://127.0.0.1:3001
```

Так тяжёлый браузерный процесс остаётся вне бинарника вашего приложения.

### 2. Встроенный (функция `embedded-browser`)

Скомпилируйте debug-сервер tairitsu *прямо в* seia. Отдельный демон не нужен:

```toml
[dependencies]
seia = { version = "0.1", features = ["embedded-browser"] }
```

```bash
seia search "rust async" --engine google --browser --embedded
```

Флаг `embedded` запускает сервер внутри процесса (см.
[`seia::embedded::start`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs)).

## Как работает браузерный поиск

Каждый браузерный поиск состоит из трёх шагов, выполняемых через HTTP-API tairitsu:

1. **Навигация** — `POST /navigate` к поисковому URL движка.
2. **Ожидание** — `POST /wait-for-selector`, пока не отрендерится контейнер
   результатов.
3. **Извлечение** — `POST /evaluate` запускает фрагмент JS, который читает из
   DOM заголовки, ссылки и аннотации.

Селекторы и шаблон URL для каждого движка находятся в
[`SearchProfile`](https://github.com/celestia-island/seia/blob/dev/src/profiles.rs):

| Профиль | URL поиска | Контейнер результатов |
| --- | --- | --- |
| `google` | `google.com/search?q=` | `div.g` |
| `baidu` | `baidu.com/s?wd=` | `div.result, div.c-container` |
| `bing_web` | `bing.com/search?q=` | `li.b_algo` |
| `yandex` | `yandex.com/search/?text=` | `li.serps-item, div.Organic` |

## Прямое использование браузерного клиента

```rust
use seia::{BrowserClient, profiles};

let client = BrowserClient::new("http://127.0.0.1:3001");

if !client.health().await.unwrap_or(false) {
    panic!("браузер tairitsu не подключён");
}

let profile = profiles::get_profile("google").unwrap();
let result = client.search("rust async", profile).await?;
```

CLI внутренне сопоставляет `--engine <имя> --browser` с соответствующим
профилем (при отсутствии совпадений откатывается к профилю `google`).
