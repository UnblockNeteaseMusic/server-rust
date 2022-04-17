//! API: `/api/v[n]/search`
//! 
//! Supported version: `v1`.

use std::sync::Arc;

use axum::{Json, Extension, response::IntoResponse};
use unm_types::Context;

use crate::executor::search::SearchPayload;

pub async fn search_v1(
    Json(payload): Json<SearchPayload>,
    Extension(default_context): Extension<Arc<Context>>
) -> impl IntoResponse {
    let context = payload.context.construct_context((*default_context).clone());
    let response = payload.search(&context).await;

    match response {
        Ok(response) => response.into_response(),
        Err(e) => e.into_response(),
    }
}
