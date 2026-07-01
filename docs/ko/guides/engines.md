# 엔진

seia는 모든 백엔드를 단일 [`Engine`](https://github.com/celestia-island/seia/blob/dev/src/engines.rs)
열거형으로 노출하므로, 백엔드를 전환해도 쿼리 코드는 영향을 받지 않습니다.

## 세 가지 실행 모드

| 모드 | 작동 방식 | 엔진 |
| --- | --- | --- |
| **API** | 검색 제공자의 HTTP API를 호출하고 JSON을 파싱합니다. | Tavily, SearXNG, Wikipedia |
| **스크랩** | HTML 결과 페이지를 가져와 검색 결과를 추출합니다. | DuckDuckGo |
| **브라우저** | 헤드리스 브라우저([tairitsu](https://github.com/celestia-island/tairitsu) 경유)를 구동하여 JS가 많은 페이지를 렌더링합니다. | Google, Baidu, Bing (웹), Yandex |

API 및 스크랩 모드는 HTTP 클라이언트만 있으면 됩니다. 브라우저 모드는
[브라우저 모드](./browser-mode.md)를 참조하세요.

## 엔진 매트릭스

| 엔진 | 열거형 값 | 모드 | 인증 | 무료 한도 |
| --- | --- | --- | --- | --- |
| DuckDuckGo | `Duckduckgo` | 스크랩 | 없음 | 무제한 |
| Wikipedia | `Wikipedia` | API | 없음 | 무제한 |
| SearXNG | `Searxng` | API | `SEARXNG_URL` | 자체 호스팅 |
| Tavily | `Tavily` | API | `TAVILY_API_KEY` | 월 1,000회 |
| Bing | `Bing` | API | `BING_SEARCH_API_KEY` | 월 1,000회 |
| Brave | `Brave` | API | `BRAVE_SEARCH_API_KEY` | 월 2,000회 |
| Google | 브라우저 프로필 | 브라우저 | tairitsu | — |
| Baidu | 브라우저 프로필 | 브라우저 | tairitsu | — |
| Bing (웹) | 브라우저 프로필 | 브라우저 | tairitsu | — |
| Yandex | 브라우저 프로필 | 브라우저 | tairitsu | — |

> Bing과 Brave의 API 백엔드는 자리표시자 구현입니다(`Engine::Bing` / `Engine::Brave`이
> "not yet implemented" 에러를 반환). 브라우저 프로필을 사용하거나
> [구현을 기여](https://github.com/celestia-island/seia)해 주세요.

## 엔진 선택

CLI:

```bash
seia search "query" --engine wikipedia
```

라이브러리:

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
client.search("query", Engine::Wikipedia).await?;
```

## 엔진 메타데이터 확인

`Engine`은 자체 메타데이터를 가지고 있어, 하드코딩 없이 UI를 구축할 수 있습니다:

```rust
use seia::Engine;

for engine in [Engine::Duckduckgo, Engine::Tavily, Engine::Bing] {
    println!("{:?}", engine);                 // duckduckgo / tavily / bing
    println!("  키 필요? {}", engine.needs_key());
    println!("  키 환경변수: {:?}", engine.api_key_env());
}
```
