use unm_resolver::engine::{SongMetadata, SongArtistMetadata, SongAlbumMetadata};

fn main() {
    let m = SongMetadata {
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
    };

    println!("{}", m.keyword());
}
