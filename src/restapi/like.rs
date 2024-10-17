use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{self, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    like::{
        model::CreateLikeDTO,
        service::{LikeService, LikeServiceError},
    },
    user::model::UserDTO,
};

#[debug_handler]
pub async fn get_likes(
    State(state): State<Arc<LikeService>>,
    Path((_, post_id)): Path<(uuid::Uuid, uuid::Uuid)>,
) -> Result<Json<Vec<UserDTO>>, LikeServiceError> {
    let likes = state.get_likes(post_id).await?;
    Ok(Json(likes))
}

#[debug_handler]
pub async fn create_like(
    State(state): State<Arc<LikeService>>,
    Path((_, post_id)): Path<(uuid::Uuid, uuid::Uuid)>,
    extract::Json(create_like_dto): extract::Json<CreateLikeDTO>,
) -> Result<(), LikeServiceError> {
    let create_like_dto = CreateLikeDTO {
        liked_by: create_like_dto.liked_by,
        liked_post: post_id,
    };
    state.crate_like(create_like_dto).await?;
    Ok(())
}

impl IntoResponse for LikeServiceError {
    fn into_response(self) -> Response {
        let body = format!("{:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
