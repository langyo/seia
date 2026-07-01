# 아키텍처

seia는 라이브러리(`src/lib.rs`)와 CLI(`src/main.rs`)를 모두 제공하는 단일 crate입니다.
설계 목표는 **하나의 쿼리 인터페이스, 여러 백엔드**입니다: 호출자가 `Engine`을 선택하면
결과를 어떻게 얻었는지와 관계없이 동일한 `SearchResult`를 받습니다.

## 모듈 맵

```
src/
├── lib.rs          공개 API 영역 + embedded-browser 서버
├── main.rs         clap CLI (search / engines)
├── engines.rs      Engine 열거형: as_str, api_key_env, needs_key, needs_browser
├── engines_impl/   각 API/스크랩 백엔드당 한 모듈
│   ├── duckduckgo.rs   스크랩 (HTML)
│   ├── wikipedia.rs    API (JSON)
│   ├── tavily.rs       API (JSON, 키 필요)
│   └── searxng.rs      API (JSON, 자체 호스팅)
├── client.rs       SearchClient + SearchOptions (API/스크랩 경로)
├── browser.rs      BrowserClient (HTTP로 tairitsu와 통신)
├── profiles.rs     SearchProfile: 엔진별 CSS 선택자 + URL 템플릿
├── extractor.rs    전체 페이지 본문 가져오기 (--fetch용)
└── result.rs       SearchResult / SearchItem / SearchMode
```

## 세 가지 실행 경로, 하나의 결과 타입

세 경로 모두 [`SearchResult`](https://github.com/celestia-island/seia/blob/dev/src/result.rs)로
수렴합니다:

```
                        ┌─ engines_impl/* (API / 스크랩) ─┐
query + Engine ─► SearchClient ─► 통합 ─► SearchResult
                        └─ browser.rs (tairitsu HTTP) ────┘
```

- **API** — `engines_impl::<engine>::search(&http, query, &opts)`가 제공자를 호출하고,
  JSON을 `SearchItem`으로 역직렬화합니다.
- **스크랩** — 동일한 시그니처이지만 HTML 결과 페이지를 파싱합니다.
- **브라우저** — `BrowserClient::search`가 tairitsu를 구동합니다; 엔진별
  `SearchProfile`이 URL과 주입된 추출 JS가 사용하는 CSS 선택자를 제공합니다.

`SearchMode`(`Api` / `Scrape` / `Browser`)는 어떤 경로가 결과를 만들어냈는지 기록하므로,
호출자는 예를 들어 캐시된 API 응답과 렌더링된 페이지를 구별할 수 있습니다.

## 디스패치

`SearchClient::search_with_options`는 `Engine`에 대한 단순 `match`입니다. 백엔드를
추가하려면: `engines_impl/`에 함수를 하나 구현하고, `Engine` 변형을 하나 추가하고,
`match` 분기를 하나 추가하면 됩니다. trait 객체나 동적 디스패치는 없습니다 — 엔진
집합은 닫혀 있고 컴파일 시점에 알려져 있으므로, API를 예측 가능하게 유지하고
바이너리를 작게 만듭니다.

## 본문 보강

`SearchOptions::fetch_content`은 직교하는 관심사입니다: 엔진이 `SearchItem`을 반환한 후,
`extractor::fetch_content`가 각 페이지를 다운로드하고 정리합니다. 이는 엔진과 무관하며
모든 모드에서 작동합니다.

## 브라우저 통합 경계

`tairitsu-packager`는 **선택적** 의존성으로, `embedded-browser` 기능으로 제어됩니다.
이것 없이 seia는 브라우저 코드가 전혀 없으며, 일반 HTTP로 외부 tairitsu 데몬에
연결합니다(`BrowserClient`). 이것을 활성화하면 `seia::embedded::start`가 프로세스 내에서
디버그 서버를 시작합니다. 이를 통해 기본 빌드는 가볍게 유지하고, 게시 가능한 crate는
무거운 브라우저 의존성을 갖지 않게 됩니다.
