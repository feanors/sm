use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{self, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::user::{
    model::{CreateUserDTO, UserDTO},
    service::{UserService, UserServiceError},
};

#[debug_handler]
pub async fn get_user(
    State(state): State<Arc<UserService>>,
    Path(user_id): Path<uuid::Uuid>,
) -> Result<Json<UserDTO>, UserServiceError> {
    let user = state.get_user(user_id).await?;
    Ok(Json(user))
}

#[debug_handler]
pub async fn create_user(
    State(state): State<Arc<UserService>>,
    extract::Json(create_user_dto): extract::Json<CreateUserDTO>,
) -> Result<Json<UserDTO>, UserServiceError> {
    let user = state.create_user(create_user_dto).await?;
    Ok(Json(user))
}

impl IntoResponse for UserServiceError {
    fn into_response(self) -> Response {
        let body = format!("{:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
