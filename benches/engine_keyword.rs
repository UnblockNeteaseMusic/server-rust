use criterion::{black_box, criterion_group, criterion_main, Criterion};
use unm_resolver::engine::{SongMetadata, SongArtistMetadata, SongAlbumMetadata};

fn criterion_benchmark(c: &mut Criterion) {
    let m = black_box(SongMetadata {
        id: 114514,
        name: "U2FsdGVkX1".to_string(),
        duration: Some(7001),
        artists: vec![
            SongArtistMetadata {
                id: 114514,
                name: "elonh".to_string(),
            },
            SongArtistMetadata {
                id: 114516,
                name: "pan93412".to_string(),
            },
        ],
        album: Some(SongAlbumMetadata {
            id: 334511,
            name: "OWOOW".to_string(),
        })
    });
    
    c.bench_function("engine > keyword()", |b| b.iter(|| m.keyword()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
