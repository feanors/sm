use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{self, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::post::{
    model::{CreatePostDTO, PostDTO, PostsDTO},
    service::{PostService, PostServiceError},
};

#[debug_handler]
pub async fn get_posts(
    State(state): State<Arc<PostService>>,
    Path(user_id): Path<uuid::Uuid>,
) -> Result<Json<PostsDTO>, PostServiceError> {
    let p = state.get_posts(user_id).await?;
    Ok(Json(p))
}

#[debug_handler]
pub async fn create_post(
    State(state): State<Arc<PostService>>,
    Path(user_id): Path<uuid::Uuid>,
    extract::Json(create_post_dto): extract::Json<CreatePostDTO>,
) -> Result<Json<PostDTO>, PostServiceError> {
    // there probably is a better way to do this, todo later
    let create_post_dto = CreatePostDTO {
        content: create_post_dto.content,
        posted_by: user_id,
    };
    let p = state.create_post(create_post_dto).await?;
    Ok(Json(p))
}

impl IntoResponse for PostServiceError {
    fn into_response(self) -> Response {
        let body = format!("{:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
