use futures::FutureExt;
use unm_resolver::{
    engine::{Artist, Context, Song},
    resolve::{batch_search, retrieve, Engine},
};
use unm_test_utils::{measure_async_function_time, set_logger};

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

    let engines = [Engine::Bilibili, Engine::YtDlp, Engine::YtDl, Engine::Migu, Engine::Kugou];
    let context = Context::default();

    let (search_time_taken, search_result) =
        measure_async_function_time(|| batch_search(&engines, &song, &context).boxed()).await;
    let search_result = search_result.expect("should has a search result");

    let (retrieve_time_taken, retrieved_result) =
        measure_async_function_time(|| retrieve(&search_result, &context).boxed()).await;
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
