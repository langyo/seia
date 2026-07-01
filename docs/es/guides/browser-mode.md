# Modo navegador

Algunos motores — Google, Baidu, Bing (la página web, no la API), Yandex — bloquean
de forma agresiva las solicitudes que no provienen de un navegador. seia los controla
a través de [tairitsu](https://github.com/celestia-island/tairitsu), un entorno de
ejecución de navegador sin interfaz. seia se comunica con la API HTTP de depuración de
tairitsu, por lo que **no** hay enlaces nativos al navegador.

## Dos formas de ejecutar tairitsu

### 1. Demonio externo (por defecto)

Ejecuta un servidor de depuración de tairitsu por separado y apunta seia hacia él:

```bash
# en una terminal
tairitsu debug --proxy http://localhost:7890

# en otra
seia search "rust async" --engine google --browser --tairitsu http://127.0.0.1:3001
```

Esto mantiene el pesado proceso del navegador fuera del binario de tu aplicación.

### 2. Integrado (la característica `embedded-browser`)

Compila el servidor de depuración de tairitsu *dentro de* seia. No hace falta un
demonio aparte:

```toml
[dependencies]
seia = { version = "0.1", features = ["embedded-browser"] }
```

```bash
seia search "rust async" --engine google --browser --embedded
```

El indicador `embedded` inicia el servidor dentro del proceso (consulta
[`seia::embedded::start`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs)).

## Cómo funciona una búsqueda en el navegador

Cada búsqueda en el navegador consta de tres pasos, todos enviados a la API HTTP de
tairitsu:

1. **Navegar** — `POST /navigate` a la URL de búsqueda del motor.
2. **Esperar** — `POST /wait-for-selector` hasta que se renderice el contenedor de
   resultados.
3. **Extraer** — `POST /evaluate` ejecuta un fragmento de JS que lee títulos, enlaces
   y resúmenes del DOM.

Los selectores y la plantilla de URL de cada motor residen en un
[`SearchProfile`](https://github.com/celestia-island/seia/blob/dev/src/profiles.rs):

| Perfil | URL de búsqueda | Contenedor de resultados |
| --- | --- | --- |
| `google` | `google.com/search?q=` | `div.g` |
| `baidu` | `baidu.com/s?wd=` | `div.result, div.c-container` |
| `bing_web` | `bing.com/search?q=` | `li.b_algo` |
| `yandex` | `yandex.com/search/?text=` | `li.serps-item, div.Organic` |

## Usar el cliente del navegador directamente

```rust
use seia::{BrowserClient, profiles};

let client = BrowserClient::new("http://127.0.0.1:3001");

if !client.health().await.unwrap_or(false) {
    panic!("tairitsu browser not connected");
}

let profile = profiles::get_profile("google").unwrap();
let result = client.search("rust async", profile).await?;
```

El CLI mapea `--engine <nombre> --browser` al perfil correspondiente internamente
(recurriendo al perfil `google` cuando no hay coincidencia).
