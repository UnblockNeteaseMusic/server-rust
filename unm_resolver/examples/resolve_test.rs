use log::LevelFilter;
use simple_logger::SimpleLogger;
use unm_resolver::{
    engine::{Artist, Context, Song},
    resolve::{resolve, Engine},
};

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let song = Song {
        name: "青花瓷".to_string(),
        artists: vec![Artist {
            name: "周杰伦".to_string(),
            ..Default::default()
        }],
        ..Default::default()
    };

    let start = std::time::Instant::now();
    let result = resolve(
        &[Engine::Bilibili, Engine::YtDlp, Engine::YtDl, Engine::Migu],
        &song,
        &Context::default(),
    )
    .await;
    let end = start.elapsed();

    println!("[resolve] 周杰伦 - 青花瓷: {:?}", result);
    println!("Time taken: {:?}", end);
}
