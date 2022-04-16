# UNM Engines

本資料夾放置 UNM 目前官方支援的音源。

## 設定

「設定」請傳入 Context 中 `config` 欄位，以下是 Rust 設定範例：

```rs
use unm_types::{ContextBuilder, config::ConfigManagerBuilder};

let config = ConfigManagerBuilder::new()
    .set("joox:cookie", "wmid=...; session_key=...")
    .build();

let context = ContextBuilder::default()
    .config(config)
    .build();
```

JavaScript 的話只需要建構 object，讓 N-API 處理即可：

```js
/** @type {Record<string, string>} */
const config = {
    "joox:cookie": "wmid=...; session_key=...",
};
```

### 可設定項目

> **設定請以各 engines 的說明文件為主**。本文件是這些說明文件統整出可以設定的項目。

| 設定鍵        | 設定值範例                                          | 說明                            |
| ------------- | --------------------------------------------------- | ------------------------------- |
| `joox:cookie` | `wmid=<your_wmid>; session_key=<your_session_key>;` | 請參見〈JOOX Cookie 設定說明〉  |
| `qq:cookie`   | WIP                                                 | （未完成）傳入 QQ 平台的 Cookie |
| `ytdl:exe`    | `youtube-dl`                                        | 請參見〈`ytdl:exe` 設定說明〉   |

### JOOX Cookie 設定說明

`joox:cookie` 是登入 JOOX 平台後，透過在 F12 → Console 輸入 `document.cookie` 取得的 Cookie。

### `ytdl:exe` 設定說明

`ytdl:exe` 是要使用的 youtube-dl 執行檔。預設值是 `yt-dlp`
