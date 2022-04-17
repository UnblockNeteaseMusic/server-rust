//! API: `/api/v[n]/retrieve`
//!
//! Supported version: `v1`.

use std::sync::Arc;

use axum::{body::StreamBody, response::IntoResponse, Extension, Json};
use tracing::info;
use unm_types::Context;

use crate::{executor::retrieve::RetrievePayload, retrieve::request_as_stream};

pub async fn retrieve_v1(
    Json(payload): Json<RetrievePayload>,
    Extension(default_context): Extension<Arc<Context>>,
) -> impl IntoResponse {
    info!(
        "[v1][Retrieve] Retrieving the song with the engine “{}”",
        payload.retrieved_song_info.source
    );

    let context = payload
        .context
        .construct_context((*default_context).clone());
    let response = match payload.retrieve(&context).await {
        Ok(response) => response,
        Err(e) => {
            return e.into_response();
        }
    };

    let retrieved_response = request_as_stream(&response).await;

    match retrieved_response {
        Ok(response) => StreamBody::new(response).into_response(),
        Err(e) => e.into_response(),
    }
}
