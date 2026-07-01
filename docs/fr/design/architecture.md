# Architecture

seia est une crate unique qui fournit à la fois une bibliothèque (`src/lib.rs`) et une CLI
(`src/main.rs`). L'objectif de conception est **une interface de requête, de nombreux backends** :
l'appelant choisit un `Engine` et obtient le même `SearchResult` quelle que soit la façon
dont le résultat a été obtenu.

## Carte des modules

```
src/
├── lib.rs          surface de l'API publique + serveur embedded-browser
├── main.rs         CLI clap (search / engines)
├── engines.rs      énumération Engine : as_str, api_key_env, needs_key, needs_browser
├── engines_impl/   un module par backend API/scraping
│   ├── duckduckgo.rs   scraping (HTML)
│   ├── wikipedia.rs    API (JSON)
│   ├── tavily.rs       API (JSON, clé)
│   └── searxng.rs      API (JSON, auto-hébergé)
├── client.rs       SearchClient + SearchOptions (chemin API/scraping)
├── browser.rs      BrowserClient (communique avec tairitsu via HTTP)
├── profiles.rs     SearchProfile : sélecteurs CSS par moteur + modèle d'URL
├── extractor.rs    récupérateur de contenu de page complète (pour --fetch)
└── result.rs       SearchResult / SearchItem / SearchMode
```

## Trois chemins d'exécution, un type de résultat

Les trois chemins convergent vers [`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs) :

```
                        ┌─ engines_impl/* (API / scraping) ─┐
query + Engine ─► SearchClient ─► unifier ─► SearchResult
                        └─ browser.rs (tairitsu HTTP) ──────┘
```

- **API** — `engines_impl::<engine>::search(&http, query, &opts)` appelle le
  fournisseur, désérialise le JSON en `SearchItem`s.
- **Scraping** — même signature, mais analyse la page de résultats HTML.
- **Navigateur** — `BrowserClient::search` pilote tairitsu ; le `SearchProfile`
  propre au moteur fournit l'URL et les sélecteurs CSS utilisés par le JS
  d'extraction injecté.

`SearchMode` (`Api` / `Scrape` / `Browser`) enregistre quel chemin a produit un
résultat, afin que les appelants puissent distinguer, par exemple, une réponse
API en cache d'une page rendue.

## Répartition

`SearchClient::search_with_options` est un `match` à plat sur `Engine`. Ajouter un
backend signifie : implémenter une fonction dans `engines_impl/`, ajouter une variante à
`Engine`, ajouter un bras au `match`. Il n'y a aucun objet trait ni répartition dynamique —
l'ensemble des moteurs est fermé et connu à la compilation, ce qui rend l'API
prévisible et le binaire léger.

## Enrichissement du contenu

`SearchOptions::fetch_content` est une préoccupation orthogonale : une fois que le moteur
a renvoyé les `SearchItem`s, `extractor::fetch_content` télécharge et nettoie chaque
page. C'est indépendant du moteur et fonctionne pour n'importe quel mode.

## Frontière d'intégration du navigateur

`tairitsu-packager` est une dépendance **facultative**, contrôlée par la fonctionnalité
`embedded-browser`. Sans elle, seia ne contient aucun code de navigateur et se
connecte à un démon tairitsu externe via du HTTP simple (`BrowserClient`).
Avec elle, `seia::embedded::start` lance le serveur de débogage dans le processus. Cela
garde la compilation par défaut légère et la crate publiable exempte de lourdes
dépendances de navigateur.
