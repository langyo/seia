# seia

**하나의 쿼리, 모든 검색 엔진.**

Rust를 위한 다중 엔진 웹 검색 라이브러리입니다. 무료 엔진은 별도 설정 없이 바로 사용할 수 있습니다.

## 소개

seia는 DuckDuckGo, Tavily, Wikipedia, SearXNG, Bing, Brave, Google, Baidu 등 여러 검색
백엔드를 하나의 인터페이스로 사용할 수 있게 해줍니다. 무료 엔진은 설정 없이 바로
작동합니다.

## 빠른 시작

### CLI

```bash
# 기본 검색 (DuckDuckGo, 무료, 키 불필요)
seia search "rust async patterns"

# Wikipedia (무료, 학술)
seia search "Klein bottle" --engine wikipedia

# JSON 출력
seia search "climate change" --json

# 프록시를 통한 검색
HTTPS_PROXY=http://localhost:7890 seia search "hello world"

# 브라우저 모드 (Google/Baidu, tairitsu 경유)
seia search "query" --engine google --browser
```

### 라이브러리

```rust
use seia::{SearchClient, Engine};

let client = SearchClient::new();
let results = client.search("rust async", Engine::DuckDuckGo).await?;
```

## 엔진

| 엔진 | 모드 | 인증 | 상태 |
|--------|------|------|--------|
| DuckDuckGo | 스크랩 | 없음 | ✅ |
| Wikipedia | API | 없음 | ✅ |
| SearXNG | API | `SEARXNG_URL` | ✅ |
| Tavily | API | `TAVILY_API_KEY` | ✅ |
| Bing | API | `BING_SEARCH_API_KEY` | 🔲 |
| Brave | API | `BRAVE_SEARCH_API_KEY` | 🔲 |
| Google | 브라우저 | tairitsu | ✅ |
| Baidu | 브라우저 | tairitsu | ✅ |
| Bing Web | 브라우저 | tairitsu | ✅ |
| Yandex | 브라우저 | tairitsu | ✅ |

브라우저 모드 엔진은 헤드리스 렌더링을 위해
[tairitsu](https://github.com/celestia-island/tairitsu)를 사용합니다. 독립 데몬을
실행하거나 `embedded-browser` 기능을 활성화하여 tairitsu를 프로세스 내에 컴파일할 수
있습니다.

## 개발

```bash
just ci          # fmt-check + clippy + test
just test        # cargo test
```

## 라이선스

SySL-1.0 (Synthetic Source License). 자세한 내용은 [LICENSE](../../LICENSE)를 참조하세요.
