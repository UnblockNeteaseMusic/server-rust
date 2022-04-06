use async_trait::async_trait;
use unm_types::{Song, Context, SongSearchInformation, SerializedIdentifier, RetrievedSongInfo};

#[async_trait]
/// The engine that can search and track the specified [`Song`].
pub trait Engine {
    /// Search an audio matched the `info`, and
    /// return the identifier for retrieving audio URL with [`retrieve`].
    async fn search<'a>(
        &self,
        info: &'a Song,
        ctx: &'a Context,
    ) -> anyhow::Result<Option<SongSearchInformation<'static>>>;
    // FIXME: allow dynamically generate the source name.

    /// Retrieve the audio URL of the specified `identifier`.
    async fn retrieve<'a>(
        &self,
        identifier: &'a SerializedIdentifier,
        ctx: &'a Context,
    ) -> anyhow::Result<RetrievedSongInfo<'static>>;
    // FIXME: allow dynamically generate the source name.
}
