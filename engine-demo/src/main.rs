use std::{borrow::Cow, sync::Arc};

use futures::FutureExt;
use mimalloc::MiMalloc;
use unm_test_utils::{measure_async_function_time, set_logger};
use unm_types::{Artist, ContextBuilder, SearchMode, Song};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    set_logger();

    let song = Song::builder()
        .name("青花瓷".to_string())
        .artists(vec![Artist::builder().name("周杰伦".to_string()).build()])
        .build();

    let context = ContextBuilder::default()
        .enable_flac(std::env::var("ENABLE_FLAC").unwrap_or_else(|_| "".into()) == "true")
        .search_mode(match std::env::var("SEARCH_MODE") {
            Ok(v) if v == "fast_first" => SearchMode::FastFirst,
            Ok(v) if v == "order_first" => SearchMode::OrderFirst,
            _ => SearchMode::FastFirst,
        })
        .build()
        .unwrap();

    let executor = {
        let mut e = unm_engine::executor::Executor::new();

        macro_rules! push_engine {
            ($engine_name:ident: $engine_struct:ident) => {
                concat_idents::concat_idents!(engine_crate = unm_engine_, $engine_name {
                    e.register(engine_crate::ENGINE_ID.into(), Arc::new(engine_crate::$engine_struct));
                })
            };
        }

        push_engine!(bilibili: BilibiliEngine);
        push_engine!(ytdl: YtDlEngine);
        push_engine!(kugou: KugouEngine);
        push_engine!(migu: MiguEngine);
        push_engine!(kuwo: KuwoEngine);

        e
    };

    let engines_to_use = std::env::var("ENGINES")
        .unwrap_or_else(|_| "bilibili ytdl kugou migu".to_string())
        .split_whitespace()
        .map(|v| Cow::Owned(v.to_string()))
        .collect::<Vec<Cow<'static, str>>>();

    let (search_time_taken, search_result) =
        measure_async_function_time(|| executor.search(&engines_to_use, &song, &context).boxed())
            .await;
    let search_result = search_result.expect("should has a search result");

    let (retrieve_time_taken, retrieved_result) =
        measure_async_function_time(|| executor.retrieve(&search_result, &context).boxed()).await;
    let retrieved_result = retrieved_result.expect("can't be retrieved");

    println!(
        "[Retrieved] 周杰伦 - 青花瓷: {} (from {})",
        retrieved_result.url, retrieved_result.source
    );
    println!(
        "Search taken {:?} while retrieve tooke {:?}.",
        search_time_taken, retrieve_time_taken
    );
}
