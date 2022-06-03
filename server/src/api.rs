use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_auth::AuthBasic;
use beater::Beater;
use librespot_core::{spotify_id::SpotifyItemType, SpotifyId};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::Mutex;

type AuthToken = String;

static SESSIONS: Lazy<Mutex<HashMap<(String, String), Beater>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn get_music_file(
    AuthBasic((username, password)): AuthBasic,
    Path(song_id): Path<String>,
) -> Response {
    if let Some(password) = password {
        let mut song_id = SpotifyId::from_base62(&song_id).unwrap();
        song_id.item_type = SpotifyItemType::Track;

        let mut sessions = SESSIONS.lock().await;
        match sessions.get_mut(&(username.clone(), password.clone())) {
            Some(beater) => {
                let song = beater.get_audio_file(song_id, None).await;
                match song {
                    Ok((song, _)) => (StatusCode::OK, song).into_response(),
                    Err(_) => StatusCode::NOT_FOUND.into_response(),
                }
            }
            None => {
                let mut beater = Beater::new(username, password).await.unwrap();
                let song = beater.get_audio_file(song_id, None).await;
                match song {
                    Ok((song, _)) => (StatusCode::OK, song).into_response(),
                    Err(_) => StatusCode::NOT_FOUND.into_response(),
                }
            }
        }
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
