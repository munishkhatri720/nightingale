use std::num::NonZeroU64;
use std::sync::Arc;
use axum::body::Body;
use axum::extract::{FromRequestParts, Query};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::Response;
use serde::Deserialize;
use tokio::sync::Mutex;
use crate::api::extractors::session::SessionExtractor;
use crate::api::state::State;
use crate::playback::player::Player;

const NOT_CONNECTED: &str = r#"{"message": "Not connected to voice"}"#;
pub const MISSING_GUILD_ID: &str = r#"{"message": "Missing guild ID"}"#;

/// Extractor that takes a guild id from the url query and resolves to the corresponding player,
/// if the guild is not provided or the player is not available, returns a 400 Bad request
/// response with the corresponding error message.
///
/// This extractor uses the [`SessionExtractor`] under the hood, and needs it to resolve first.
pub struct PlayerExtractor(pub Arc<Mutex<Player>>);

#[async_trait::async_trait]
impl FromRequestParts<State> for PlayerExtractor {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &State) -> Result<Self, Self::Rejection> {
        #[derive(Deserialize)]
        struct GuildQuery {
            guild_id: NonZeroU64
        }

        let SessionExtractor(session) = SessionExtractor::from_request_parts(parts, state).await?;
        let Query(query) = Query::<GuildQuery>::from_request_parts(parts, state).await
            .map_err(|_| {
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header(
                        axum::http::header::CONTENT_TYPE,
                        super::super::APPLICATION_JSON
                    )
                    .body(Body::from(MISSING_GUILD_ID))
                    .unwrap()
            })?;

        let Some(player) = session.playback.get_player(query.guild_id) else {
            return Err(
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header(
                        axum::http::header::CONTENT_TYPE,
                        super::super::APPLICATION_JSON
                    )
                    .body(Body::from(NOT_CONNECTED))
                    .unwrap()
            );
        };

        Ok(PlayerExtractor(player))
    }
}