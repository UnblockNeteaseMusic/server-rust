use std::{time::Duration, future::Future, pin::Pin};

/// Measure the time taken by the given closure.
#[inline]
pub fn measure_function_time<T>(func: impl Fn() -> T) -> (Duration, T) {
    let start = std::time::Instant::now();
    let response = func();

    (start.elapsed(), response)
}

/// Measure the time taken by the given asynchronous closure.
#[inline]
pub async fn measure_async_function_time<'a, T>(func: impl Fn() -> Pin<Box<dyn Future<Output = T> + 'a>>) -> (Duration, T) {
    let start = std::time::Instant::now();
    let response = func().await;

    (start.elapsed(), response)
}
