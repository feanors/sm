use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{debug_handler, Json, Router};
use dashmap::DashMap;
use futures::StreamExt;
use like::model::LikeEvent;

use std::error;
use thiserror::Error;
use tokio::task;

use std::sync::Arc;

use sm::like;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let posts_like_count: Arc<DashMap<uuid::Uuid, u64>> = Arc::new(DashMap::new());

    let nats_client: async_nats::Client = async_nats::connect("demo.nats.io").await?;

    let mut subscriber = nats_client.subscribe("likes").await?;

    let post_like_count_clone = posts_like_count.clone();
    task::spawn(async move {
        while let Some(message) = subscriber.next().await {
            let like = match serde_json::from_slice(&message.payload) {
                Ok::<LikeEvent, _>(e) => e,
                Err(_) => continue,
            };

            let current_like_count = match post_like_count_clone.get(&like.liked_post) {
                Some(v) => *v,
                None => 0,
            };
            post_like_count_clone.insert(like.liked_post, current_like_count + 1);
        }
    });

    let app: Router<_> = Router::new()
        .route("/posts/:id/likes/count", get(get_like_count))
        .with_state(posts_like_count);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Error, Debug)]
#[error("{msg}")]
pub struct NotFoundError {
    msg: String,
}

#[debug_handler]
pub async fn get_like_count(
    State(state): State<Arc<DashMap<uuid::Uuid, u64>>>,
    Path(post_id): Path<uuid::Uuid>,
) -> Result<Json<u64>, NotFoundError> {
    match state.get(&post_id) {
        Some(res) => Ok(axum::Json(*res)),
        None => Err(NotFoundError {
            msg: "post with id: {} not found".to_string(),
        }),
    }
}

impl IntoResponse for NotFoundError {
    fn into_response(self) -> Response {
        let body = format!("{:?}", self);
        (StatusCode::NOT_FOUND, body).into_response()
    }
}
