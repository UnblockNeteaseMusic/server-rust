use futures::FutureExt;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::{future::Future, pin::Pin, time::Duration};
use unm_engine::interface::Engine;
use unm_types::{Artist, Context, Song};

/// Measure the time taken by the given closure.
#[inline]
pub fn measure_function_time<T>(func: impl Fn() -> T) -> (Duration, T) {
    let start = std::time::Instant::now();
    let response = func();

    (start.elapsed(), response)
}

/// Measure the time taken by the given asynchronous closure.
#[inline]
pub async fn measure_async_function_time<'a, T>(
    func: impl Fn() -> Pin<Box<dyn Future<Output = T> + 'a>>,
) -> (Duration, T) {
    let start = std::time::Instant::now();
    let response = func().await;

    (start.elapsed(), response)
}

#[inline]
pub fn set_logger() {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();
}

pub async fn engine_example_wrapper(engine: impl Engine) {
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

    let (search_time_taken, search_result) =
        measure_async_function_time(|| engine.search(&song, &context).boxed()).await;
    let identifier = search_result
        .expect("failed to search")
        .expect("should has a search result")
        .identifier;

    let (retrieve_time_taken, retrieved_result) =
        measure_async_function_time(|| engine.retrieve(&identifier, &context).boxed()).await;
    let retrieved_result = retrieved_result.expect("can't be retrieved");

    println!(
        "[Retrieved] 周杰伦 - 青花瓷: {} (from {})",
        retrieved_result.url, retrieved_result.source
    );
    println!(
        "Search taken {:?} while retrieve took {:?}.",
        search_time_taken, retrieve_time_taken
    );
}
