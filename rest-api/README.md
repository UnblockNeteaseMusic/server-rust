# UNM (Rust) 的 RESTful API

這個 API 可以方便您使用傳統 HTTP API 的呼叫形式使用 UNM 的 Engine 及 Executor。

## 安裝

### 使用預編譯版本

> WIP

### 從 crates.io 編譯安裝

```sh
cargo install unm_rest_api
unm_rest_api
```

### 從本 codebase 編譯安裝

```sh
cargo build --release --bin unm_rest_api
```

## 使用

### 環境變數

| 環境變數        | 說明                          | 範例值         |
| --------------- | ----------------------------- | -------------- |
| `RUST_LOG`      | 日誌輸出的等級。預設是 `info` | `debug`        |
| `SERVE_ADDRESS` | 啟動伺服器的 IP:port          | `0.0.0.0:1234` |

#### `RUST_LOG` 可使用的等級

- `trace`
- `debug`
- `info`
- `warn`
- `error`
- `slient`

### API 說明文件

請參見 [docs/api.md](docs/api.md)

### `config.toml` 設定說明

請參見 [docs/configure.md](docs/configure.md)


## 授權條款

LGPL-3.0-or-later
