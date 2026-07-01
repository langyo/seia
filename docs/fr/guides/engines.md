# Moteurs

seia expose chaque backend via l'énumération unique [`Engine`](https://github.com/celestia-island/seia/blob/dev/src/engines.rs),
de sorte que changer de backend ne touche jamais à votre code de requête.

## Trois modes d'exécution

| Mode | Fonctionnement | Moteurs |
| --- | --- | --- |
| **API** | Appelle l'API HTTP d'un fournisseur de recherche et analyse le JSON. | Tavily, SearXNG, Wikipedia |
| **Scraping** | Récupère la page de résultats HTML et en extrait les correspondances. | DuckDuckGo |
| **Navigateur** | Pilote un navigateur headless (via [tairitsu](https://github.com/celestia-island/tairitsu)) pour rendre les pages riches en JS. | Google, Baidu, Bing (web), Yandex |

Les modes API et scraping ne nécessitent qu'un client HTTP. Le mode navigateur est
décrit dans [Mode navigateur](./browser-mode.md).

## Matrice des moteurs

| Moteur | Valeur d'énumération | Mode | Authentification | Offre gratuite |
| --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | Scraping | aucun | illimité |
| Wikipedia | `Wikipedia` | API | aucun | illimité |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | auto-hébergé |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 1 000 / mois |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 1 000 / mois |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 2 000 / mois |
| Google | profil navigateur | Navigateur | tairitsu | — |
| Baidu | profil navigateur | Navigateur | tairitsu | — |
| Bing (web) | profil navigateur | Navigateur | tairitsu | — |
| Yandex | profil navigateur | Navigateur | tairitsu | — |

> Les backends API de Bing et Brave sont des squelettes (`Engine::Bing` / `Engine::Brave`
> renvoient une erreur « not yet implemented »). Utilisez les profils navigateur ou
> [contribuez](https://github.com/celestia-island/seia) une implémentation.

## Choisir un moteur

CLI :

```bash
seia search "query" --engine wikipedia
```

Bibliothèque :

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;
```

## Inspecter les métadonnées d'un moteur

`Engine` porte ses propres métadonnées, vous pouvez donc construire des interfaces sans rien coder en dur :

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing
    println!("  clé requise ? {}", engine.needs_key());
    println!("  var. d'env : {:?}", engine.api_key_env());
}
```
