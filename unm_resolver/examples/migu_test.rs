use unm_resolver::engine::{Artist, Engine, Song};

#[tokio::main]
async fn main() {
    let engine = unm_resolver::engine::migu::MiguEngine;
    let song = Song {
        name: "青花瓷".to_string(),
        artists: vec![Artist {
            name: "周杰伦".to_string(),
            ..Default::default()
        }],
        ..Default::default()
    };

    let start = std::time::Instant::now();
    let result = engine.check(&song, None).await.unwrap();
    let end = start.elapsed();

    println!("周杰伦 - 青花瓷: {:?}", result);
    println!("Time taken: {:?}", end);
}
