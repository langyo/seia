# Arquitectura

seia es un único crate que incluye tanto una biblioteca (`src/lib.rs`) como un CLI
(`src/main.rs`). El objetivo de diseño es **una superficie de consulta, múltiples
backends**: quien llama elige un `Engine` y obtiene el mismo `SearchResult`
independientemente de cómo se haya obtenido el resultado.

## Mapa de módulos

```
src/
├── lib.rs          superficie de la API pública + servidor embedded-browser
├── main.rs         clap CLI (search / engines)
├── engines.rs      Engine enum: as_str, api_key_env, needs_key, needs_browser
├── engines_impl/   un módulo por backend de API/raspado
│   ├── duckduckgo.rs   raspado (HTML)
│   ├── wikipedia.rs    API (JSON)
│   ├── tavily.rs       API (JSON, clave)
│   └── searxng.rs      API (JSON, autoalojado)
├── client.rs       SearchClient + SearchOptions (ruta API/raspado)
├── browser.rs      BrowserClient (se comunica con tairitsu por HTTP)
├── profiles.rs     SearchProfile: selectores CSS por motor + plantilla de URL
├── extractor.rs    obtenedor de contenido de página completa (para --fetch)
└── result.rs       SearchResult / SearchItem / SearchMode
```

## Tres rutas de ejecución, un tipo de resultado

Las tres rutas convergen en
[`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs):

```
                       ┌─ engines_impl/* (API / raspado) ─┐
query + Engine ─► SearchClient ─► unificar ─► SearchResult
                       └─ browser.rs (tairitsu HTTP) ────┘
```

- **API** — `engines_impl::<engine>::search(&http, query, &opts)` llama al proveedor y
  deserializa el JSON en `SearchItem`.
- **Raspado** — misma firma, pero analiza la página HTML de resultados.
- **Navegador** — `BrowserClient::search` controla tairitsu; el `SearchProfile` de
  cada motor aporta la URL y los selectores CSS que usa el JS de extracción inyectado.

`SearchMode` (`Api` / `Scrape` / `Browser`) registra qué ruta produjo un resultado, de
modo que quien llama pueda distinguir, p. ej., una respuesta en caché de la API de una
página renderizada.

## Despacho

`SearchClient::search_with_options` es un `match` plano sobre `Engine`. Añadir un
backend significa: implementar una función en `engines_impl/`, añadir una variante de
`Engine`, añadir un brazo del `match`. No hay trait object ni despacho dinámico: el
conjunto de motores es cerrado y conocido en tiempo de compilación, lo que mantiene la
API predecible y el binario pequeño.

## Enriquecimiento de contenido

`SearchOptions::fetch_content` es una preocupación ortogonal: una vez que el motor
devuelve los `SearchItem`, `extractor::fetch_content` descarga y limpia cada página.
Esto es independiente del motor y funciona en cualquier modo.

## Límite de integración del navegador

`tairitsu-packager` es una dependencia **opcional**, condicionada por la característica
`embedded-browser`. Sin ella, seia no tiene nada de código de navegador y se conecta a
un demonio externo de tairitsu por HTTP plano (`BrowserClient`). Con ella,
`seia::embedded::start` inicia el servidor de depuración dentro del proceso. Esto
mantiene la compilación por defecto ligera y el crate publicable libre de dependencias
pesadas de navegador.
