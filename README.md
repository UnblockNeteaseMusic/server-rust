# `UnblockNeteaseMusic/server-rust`

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FUnblockNeteaseMusic%2Fserver-rust.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FUnblockNeteaseMusic%2Fserver-rust?ref=badge_shield)

Rust 版本的 [UnblockNeteaseMusic/server](https://github.com/UnblockNeteaseMusic/server)，以效能、穩定性及可維護性為目標。

> 目前使用者文件及開發文件 **仍在撰寫**，在此之前有任何問題，歡迎開 Discussion 詢問。

## 架構

> 註：目前 UnblockNeteaseMusic/server 只實作 engine/resolver 的部分。

- `crypto`：與加密相關的函式庫，如 md5、aes128 等。
- `engine-base`：Engine 的抽象部分，包含一個 Engine 應有的介面、整合所有 Engines 的 Executor 等。
- `engines`
  - 這目錄底下的是官方提供的引擎，所有引擎都是選擇性依賴、使用的。
  - 您可以自行實作其他平台，並發佈到 crates.io（當然也歡迎發 PR 讓引擎納入本 codebase 一併管理）。
  - 每個 Engine 都有 `examples` 方便測試單一引擎模組。如您是開發者，可仿造其它引擎，撰寫自己的 example。
- `request`：UNM 的 reqwest 封裝，自動帶上 `User-Agent` 等 headers。
- `selector`：包含選擇最適音樂項目的演算法。
- `types`：UNM 的各種基礎類型（如 `Song`、`Artist`⋯⋯）
- `test-utils`：方便撰寫測試方法及 demo 的工具集。
- `napi`：Node.js 的 UNM (Rust) 綁定。
  - 這個綁定因 napi 限制，目前不像 Rust 版一樣有方便的擴充系統。
  - 原則上是啟用 `engines/` 底下的所有引擎。
- `demo`：用來測試及展示 UNM (Rust) 的 demo 程式。
  - 啟動 Demo：`cargo run --release --bin unm_engine_demo`

## 使用

### Rust 函式庫

> 可以參考 `engine-demo` 的用法～

首先，您需要從 <https://crates.io> 引用至少三個元件：

- `unm_engine`：包含並行查詢音源結果的 Executor。
- `unm_engine_[想要的引擎]`：用來從音源搜尋的引擎。
- `unm_types`：UNM 的基礎類型。撰寫函數時十分需要。

然後，我們可以註冊音源：

```rust
use unm_engine::executor::Executor;
use unm_engine_bilibili::{BilibiliEngine, ENGINE_ID as BILIBILI_ENGINE_ID};

let mut executor = Executor::new();
executor.register(BILIBILI_ENGINE_ID, BilibiliEngine::new());
```

接著就可以直接使用 executor 提供的方法搜尋及取回結果了：

```rust
use unm_types::{Song, Artist, Context};

let context = Context::default();

let search_result = executor.search(&[BILIBILI_ENGINE_ID], Song {
  id: "".to_string(),
  name: "TT",
  artists: vec![
    Artist {
      id: "".to_string(),
      name: "Twice",
    },
  ],
}, &context).await?;

let result = executor.retrieve(&search_result, &context).await?;
```

### TypeScript (JS) 函式庫

請參考 [napi 的 README.md](https://github.com/UnblockNeteaseMusic/server-rust/blob/main/napi/README.md)。

## 貢獻

### 檢查程式碼的相關命令

```bash
cargo check  # 檢查程式碼是否合法 (valid)
cargo test   # 執行本 codebase 的所有 Tests
cargo clippy # Rust linter
```

UNM (Rust) 的 CI 也會在程式碼 push 後自動執行上述命令，
進行程式碼測試與檢查。

### 貢獻引擎後的建議事項

引擎的 crate 名稱格式是：`unm_engine_[引擎名稱]`，放置在 `/engines/[引擎名稱]` 目錄。

建議仿照其它引擎，在 `engine-demo` 和 `napi` 註冊自己的音源。
註冊音源有 macro 協助，語法目前是這樣的：

```rust
push_engine!([引擎名稱]: [引擎實體]);
```

範例如下：

```rust
push_engine!(bilibili: BilibiliEngine);
```

## 授權條款

This project is licensed under [LGPL-3.0-only](https://spdx.org/licenses/LGPL-3.0-only.html).

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FUnblockNeteaseMusic%2Fserver-rust.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FUnblockNeteaseMusic%2Fserver-rust?ref=badge_large)
