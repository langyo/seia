# Motores

seia expone cada backend a través del único enum
[`Engine`](https://github.com/celestia-island/seia/blob/dev/src/engines.rs),
por lo que cambiar de backend nunca afecta a tu código de consulta.

## Tres modos de ejecución

| Modo | Cómo funciona | Motores |
| --- | --- | --- |
| **API** | Llama a la API HTTP del proveedor de búsqueda y analiza el JSON. | Tavily, SearXNG, Wikipedia |
| **Raspado** | Descarga la página HTML de resultados y extrae las coincidencias. | DuckDuckGo |
| **Navegador** | Controla un navegador sin interfaz (vía [tairitsu](https://github.com/celestia-island/tairitsu)) para renderizar páginas con mucho JS. | Google, Baidu, Bing (web), Yandex |

Los modos API y raspado no necesitan más que un cliente HTTP. El modo navegador se
describe en [Modo navegador](./browser-mode.md).

## Matriz de motores

| Motor | Valor del enum | Modo | Autenticación | Nivel gratuito |
| --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | Raspado | ninguno | ilimitado |
| Wikipedia | `Wikipedia` | API | ninguno | ilimitado |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | autoalojado |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000 / mes |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000 / mes |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000 / mes |
| Google | perfil de navegador | Navegador | tairitsu | — |
| Baidu | perfil de navegador | Navegador | tairitsu | — |
| Bing (web) | perfil de navegador | Navegador | tairitsu | — |
| Yandex | perfil de navegador | Navegador | tairitsu | — |

> Los backends de API de Bing y Brave son esqueletos (`Engine::Bing` / `Engine::Brave`
> devuelven un error de «aún no implementado»). Usa los perfiles de navegador o
> [contribuye](https://github.com/celestia-island/seia) con una implementación.

## Seleccionar un motor

CLI:

```bash
seia search "query" --engine wikipedia
```

Biblioteca:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;
```

## Inspeccionar los metadatos del motor

`Engine` incorpora sus propios metadatos, por lo que puedes construir interfaces de
usuario sin codificarlos a mano:

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing
    println!("  needs key? {}", engine.needs_key());
    println!("  key env:    {:?}", engine.api_key_env());
}
```
