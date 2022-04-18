# UNM REST API 之組態設定說明

此 API 的組態設定檔名為 `config.toml`，應放置與 REST API 同路徑。

## 欄位說明

- `[context]`：即 [unm_types::Context](https://docs.rs/unm_types/latest/unm_types/struct.Context.html)。
- `[context.config]`：`unm_types::Context` 底下的 `config` 欄位。

## 範例設定

```toml
# The default context.
[context]
# The proxy URI to request services.
# Comment this line to disable Proxy feature.
# proxy_uri = ""

# Should we retrieve FLAC by default?
# enable_flac = false

# The search mode for waiting the response.
# Can be `fast_first` or `order_first`.
# search_mode = "fast_first"

# The default config for engines.
[context.config]
# "joox:cookie" = "..."

# Note that we don't allow users changing this value
# for the security concerns.
# "ytdl:exe" = "..."
```
