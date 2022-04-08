use std::sync::Arc;

use futures::FutureExt;
use unm_test_utils::{measure_async_function_time, set_logger};
use unm_types::{Artist, Context, Song};

#[tokio::main]
async fn main() {
    set_logger();
    let song = Song {
        name: "青花瓷".to_string(),
        artists: vec![Artist {
            name: "周杰伦".to_string(),
            ..Default::default()
        }],
        ..Default::default()
    };

    let context = Context::default();
    let executor = {
        let mut e = unm_engine::executor::Executor::new();
        e.register(unm_engine_bilibili::ENGINE_ID, Arc::new(unm_engine_bilibili::BilibiliEngine));
        e.register(unm_engine_ytdl::ENGINE_ID, Arc::new(unm_engine_ytdl::YtDlEngine));
        e.register(unm_engine_kugou::ENGINE_ID, Arc::new(unm_engine_kugou::KugouEngine));
        e.register(unm_engine_migu::ENGINE_ID, Arc::new(unm_engine_migu::MiguEngine));
        e
    };

    let (search_time_taken, search_result) =
        measure_async_function_time(|| executor.search(&[unm_engine_bilibili::ENGINE_ID, unm_engine_ytdl::ENGINE_ID, unm_engine_kugou::ENGINE_ID, unm_engine_migu::ENGINE_ID], &song, &context).boxed()).await;
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
