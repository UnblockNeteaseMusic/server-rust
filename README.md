# 【開發中】`unm-server-rust`

<!-- TODO: 開發完成後統一轉換成簡體中文（或是雙語系） -->

Rust 版本的 [UnblockNeteaseMusic/server](https://github.com/UnblockNeteaseMusic/server)
，以效能、穩定性及可維護性為目標。

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FUnblockNeteaseMusic%2Fserver-rust.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FUnblockNeteaseMusic%2Fserver-rust?ref=badge_shield)
[![Dependency Status](https://deps.rs/repo/github/UnblockNeteaseMusic/server-rust/status.svg)](https://deps.rs/repo/github/UnblockNeteaseMusic/server-rust)
![License: LGPL-3.0](https://shields.io/github/license/UnblockNeteaseMusic/server-rust)
![Line count](https://shields.io/tokei/lines/github/UnblockNeteaseMusic/server-rust)

## 安裝

### 最新版本

#### 下載二進位檔案

<!-- TODO: Release -->

前往 Actions 分頁找到 “Build binaries for UNM“，點開後可從 Artifacts 中
選擇符合您電腦架構的版本。假如沒找到，您可以參考下文自行編譯。

#### 自行編譯

0. 使用 `rustup` 安裝 Rust toolchain。
   `stable` 和 `beta` 應該都行。
1. clone 本儲存庫。
    ```bash
    git clone https://github.com/UnblockNeteaseMusic/server-rust.git
    ```
2. 進入資料夾後開始編譯。
    ```bash
   cd server-rust
   cargo build # 也可以加上 --production 編譯最佳化過的版本
    ```
3. 進入 `target` 的 `debug` 資料夾，執行 `unm_cli` 即可。
   ```bash
   cd ./target/debug
   ./unm_cli
   ```

## 貢獻

### 檢查程式碼的相關命令

```bash
cargo check  # 檢查程式碼是否合法 (valid)
cargo test   # 執行本 codebase 的所有 Tests
cargo clippy # Rust linter
```

`unm-server-rust` 的 CI 也會在程式碼 push 後自動執行上述命令，
進行程式碼測試與檢查。

## 授權條款

This project is licensed under [LGPL-3.0-only](https://spdx.org/licenses/LGPL-3.0-only.html).

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FUnblockNeteaseMusic%2Fserver-rust.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FUnblockNeteaseMusic%2Fserver-rust?ref=badge_large)
