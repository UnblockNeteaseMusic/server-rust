use criterion::{black_box, criterion_group, criterion_main, Criterion};
use unm_resolver::engine::{Song, Artist, Album};

fn criterion_benchmark(c: &mut Criterion) {
    let m = black_box(Song {
        ncm_id: Some(114514),
        name: "U2FsdGVkX1".to_string(),
        duration: Some(7001),
        artists: vec![
            Artist {
                ncm_id: Some(114514),
                name: "elonh".to_string(),
            },
            Artist {
                ncm_id: Some(114516),
                name: "pan93412".to_string(),
            },
        ],
        album: Some(Album {
            ncm_id: Some(334511),
            name: "OWOOW".to_string(),
            ..Default::default()
        })
    });
    
    c.bench_function("engine > keyword()", |b| b.iter(|| m.keyword()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
