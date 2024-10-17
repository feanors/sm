use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{self, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    friendship::{
        model::AddFriendDTO,
        service::{FriendshipService, FriendshipServiceError},
    },
    user::model::UserDTO,
};

#[debug_handler]
pub async fn get_friends(
    State(state): State<Arc<FriendshipService>>,
    Path(user_id): Path<uuid::Uuid>,
) -> Result<Json<Vec<UserDTO>>, FriendshipServiceError> {
    let friends = state.get_friends(user_id).await?;
    Ok(Json(friends))
}

#[debug_handler]
pub async fn add_friend(
    State(state): State<Arc<FriendshipService>>,
    Path(user_id): Path<uuid::Uuid>,
    extract::Json(add_friend_dto): extract::Json<AddFriendDTO>,
) -> Result<(), FriendshipServiceError> {
    let add_friend_dto = AddFriendDTO {
        user1: user_id,
        user2: add_friend_dto.user2,
    };
    state.add_friend(add_friend_dto).await?;
    Ok(())
}

impl IntoResponse for FriendshipServiceError {
    fn into_response(self) -> Response {
        let body = format!("{:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
