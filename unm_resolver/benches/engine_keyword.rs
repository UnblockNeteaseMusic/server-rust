use criterion::{black_box, criterion_group, criterion_main, Criterion};
use unm_resolver::engine::{Album, Artist, Song};

fn criterion_benchmark(c: &mut Criterion) {
    let m = black_box(Song {
        id: "114514".to_string(),
        name: "U2FsdGVkX1".to_string(),
        duration: Some(7001),
        artists: vec![
            Artist {
                id: "114514".to_string(),
                name: "elonh".to_string(),
            },
            Artist {
                id: "114516".to_string(),
                name: "pan93412".to_string(),
            },
        ],
        album: Some(Album {
            id: "334511".to_string(),
            name: "OWOOW".to_string(),
        }),
        ..Default::default()
    });

    c.bench_function("engine > keyword()", |b| b.iter(|| m.keyword()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
