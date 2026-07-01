# Mode navigateur

Certains moteurs — Google, Baidu, Bing (la page web, pas l'API), Yandex —
bloquent agressivement les requêtes non émises par un navigateur. seia les pilote via
[tairitsu](https://github.com/celestia-island/tairitsu), un runtime de navigateur
headless. seia s'adresse à l'API de débogage HTTP de tairitsu, il n'y a donc **aucune**
liaison native avec un navigateur.

## Deux façons d'exécuter tairitsu

### 1. Démon externe (par défaut)

Lancez un serveur de débogage tairitsu séparément et pointez seia vers lui :

```bash
# dans un terminal
tairitsu debug --proxy http://localhost:7890

# dans un autre
seia search "rust async" --engine google --browser --tairitsu http://127.0.0.1:3001
```

Cela maintient le lourd processus du navigateur hors de votre binaire applicatif.

### 2. Embarqué (la fonctionnalité `embedded-browser`)

Compilez le serveur de débogage de tairitsu *au sein de* seia. Aucun démon séparé requis :

```toml
[dependencies]
seia = { version = "0.1", features = ["embedded-browser"] }
```

```bash
seia search "rust async" --engine google --browser --embedded
```

Le drapeau `embedded` lance le serveur dans le processus (voir
[`seia::embedded::start`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs)).

## Comment fonctionne une recherche par navigateur

Chaque recherche par navigateur se déroule en trois étapes, toutes envoyées à l'API HTTP de tairitsu :

1. **Naviguer** — `POST /navigate` vers l'URL de recherche du moteur.
2. **Attendre** — `POST /wait-for-selector` jusqu'au rendu du conteneur de résultats.
3. **Extraire** — `POST /evaluate` exécute un extrait de JS qui lit les titres, liens
   et résumés depuis le DOM.

Les sélecteurs et le modèle d'URL de chaque moteur se trouvent dans un
[`SearchProfile`](https://github.com/celestia-island/seia/blob/dev/src/profiles.rs) :

| Profile | URL de recherche | Conteneur de résultats |
| --- | --- | --- |
| `google` | `google.com/search?q=` | `div.g` |
| `baidu` | `baidu.com/s?wd=` | `div.result, div.c-container` |
| `bing_web` | `bing.com/search?q=` | `li.b_algo` |
| `yandex` | `yandex.com/search/?text=` | `li.serps-item, div.Organic` |

## Utiliser directement le client navigateur

```rust
use seia::{BrowserClient, profiles};

let client = BrowserClient::new("http://127.0.0.1:3001");

if !client.health().await.unwrap_or(false) {
    panic!("tairitsu browser not connected");
}

let profile = profiles::get_profile("google").unwrap();
let result = client.search("rust async", profile).await?;
```

La CLI mappe en interne `--engine <nom> --browser` au profil correspondant
(en repli sur le profil `google` lorsqu'aucun ne correspond).
