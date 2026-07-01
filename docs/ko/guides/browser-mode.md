# 브라우저 모드

일부 엔진 — Google, Baidu, Bing (API가 아닌 웹 페이지), Yandex — 은 비브라우저 요청을
강력하게 차단합니다. seia는 헤드리스 브라우저 런타임인
[tairitsu](https://github.com/celestia-island/tairitsu)를 통해 이들을 구동합니다.
seia는 tairitsu의 HTTP 디버그 API를 사용하므로 네이티브 브라우저 바인딩은 **없습니다**.

## tairitsu를 실행하는 두 가지 방법

### 1. 외부 데몬 (기본값)

애플리케이션 외부에서 tairitsu 디버그 서버를 실행하고 seia가 그것을 가리키게 합니다:

```bash
# 한 터미널에서
tairitsu debug --proxy http://localhost:7890

# 다른 터미널에서
seia search "rust async" --engine google --browser --tairitsu http://127.0.0.1:3001
```

이렇게 하면 무거운 브라우저 프로세스가 애플리케이션 바이너리 밖에 머무릅니다.

### 2. 내장 (`embedded-browser` 기능)

tairitsu의 디버그 서버를 seia *안에* 컴파일합니다. 별도의 데몬이 필요 없습니다:

```toml
[dependencies]
seia = { version = "0.1", features = ["embedded-browser"] }
```

```bash
seia search "rust async" --engine google --browser --embedded
```

`embedded` 플래그는 프로세스 내 서버를 시작합니다
([`seia::embedded::start`](https://github.com/celestia-island/seia/blob/dev/src/lib.rs) 참조).

## 브라우저 검색의 작동 방식

각 브라우저 검색은 세 단계로 이루어지며, 모두 tairitsu HTTP API에 대해 수행됩니다:

1. **탐색** — `POST /navigate`로 엔진의 검색 URL로 이동합니다.
2. **대기** — `POST /wait-for-selector`로 결과 컨테이너가 렌더링될 때까지 기다립니다.
3. **추출** — `POST /evaluate`가 JS 코드를 실행하여 DOM에서 제목, 링크, 요약을
   읽어옵니다.

각 엔진의 선택자와 URL 템플릿은
[`SearchProfile`](https://github.com/celestia-island/seia/blob/dev/src/profiles.rs)에
정의되어 있습니다:

| 프로필 | 검색 URL | 결과 컨테이너 |
| --- | --- | --- |
| `google` | `google.com/search?q=` | `div.g` |
| `baidu` | `baidu.com/s?wd=` | `div.result, div.c-container` |
| `bing_web` | `bing.com/search?q=` | `li.b_algo` |
| `yandex` | `yandex.com/search/?text=` | `li.serps-item, div.Organic` |

## 브라우저 클라이언트 직접 사용

```rust
use seia::{BrowserClient, profiles};

let client = BrowserClient::new("http://127.0.0.1:3001");

if !client.health().await.unwrap_or(false) {
    panic!("tairitsu 브라우저가 연결되지 않음");
}

let profile = profiles::get_profile("google").unwrap();
let result = client.search("rust async", profile).await?;
```

CLI는 내부적으로 `--engine <이름> --browser`를 일치하는 프로필에 매핑합니다
(일치하는 것이 없으면 `google` 프로필로 폴백).
