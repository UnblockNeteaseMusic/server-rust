pub mod engines;
pub mod types;

use engines::{Engine, RustEngine};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use types::{Context, RetrievedSongInfo, Song, SongSearchInformation};

use unm_resolver::resolve::{batch_search as r_batch_search, retrieve as r_retrieve};

/// [napi-rs] Batch search the `song` with the specified engines parallelly.
#[napi]
pub async fn batch_search(
    engines: Vec<Engine>,
    info: Song,
    context: Context,
) -> Result<SongSearchInformation> {
    let engines = engines
        .into_iter()
        .map(|e| e.into())
        .collect::<Vec<RustEngine>>();

    r_batch_search(&engines, &info.into(), &context.to_unm_context())
        .await
        .map(|v| v.into())
        .map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to resolve: {:?}", e),
            )
        })
}

/// [napi-rs] Retrieve the song with [`SongSearchInformation`].
#[napi]
pub async fn retrieve(info: SongSearchInformation, context: Context) -> Result<RetrievedSongInfo> {
    r_retrieve(&info.into(), &context.to_unm_context())
        .await
        .map(|v| v.into())
        .map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to retrieve: {:?}", e),
            )
        })
}
