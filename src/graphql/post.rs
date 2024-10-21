use crate::graphql::schema::GQLError;
use crate::like::service::{LikeService, LikeServiceError};
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use diesel_async::pooled_connection::deadpool::PoolError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::graphql::user::UserDTO;

#[derive(Serialize, SimpleObject)]
#[graphql(complex)]
pub struct PostDTO {
    pub id: uuid::Uuid,
    pub posted_by: uuid::Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[ComplexObject]
impl PostDTO {
    async fn likes(&self, ctx: &Context<'_>) -> Result<Vec<UserDTO>, GQLError> {
        let like_service = match ctx.data::<Arc<LikeService>>() {
            Ok(p) => p,
            Err(_) => {
                return Err(GQLError::LikeService(LikeServiceError::DBPool(
                    PoolError::Closed,
                )))
            }
        };
        let likes = like_service.get_likes(self.id).await?;
        Ok(likes.into_iter().map(|l| l.into()).collect())
    }
}

#[derive(InputObject, Deserialize)]
pub struct CreatePostDTO {
    pub posted_by: uuid::Uuid,
    pub content: String,
}
