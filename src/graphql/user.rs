use crate::friendship::service::FriendshipService;
use crate::graphql::post::PostDTO;
use crate::graphql::schema::GQLError;
use crate::post::service::PostService;
use crate::user::service::UserServiceError;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use diesel_async::pooled_connection::deadpool::PoolError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use chrono::{DateTime, Utc};

#[derive(Serialize, SimpleObject)]
#[graphql(complex)]
pub struct UserDTO {
    pub id: uuid::Uuid,
    pub username: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[ComplexObject]
impl UserDTO {
    pub async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<PostDTO>, GQLError> {
        let post_service = match ctx.data::<Arc<PostService>>() {
            Ok(p) => p.to_owned(),
            Err(e) => {
                println!("{e:?}");
                return Err(GQLError::UserService(UserServiceError::DBPool(
                    PoolError::Closed,
                )));
            }
        };
        let a = post_service.get_posts(self.id).await?;
        Ok(a.into_iter().map(|p| p.into()).collect())
    }

    pub async fn friends(&self, ctx: &Context<'_>) -> Result<Vec<UserDTO>, GQLError> {
        let friendship_service = match ctx.data::<Arc<FriendshipService>>() {
            Ok(p) => p.to_owned(),
            Err(e) => {
                println!("{e:?}");
                return Err(GQLError::UserService(UserServiceError::DBPool(
                    PoolError::Closed,
                )));
            }
        };
        let a = friendship_service.get_friends(self.id).await?;
        Ok(a.into_iter().map(|p| p.into()).collect())
    }
}

#[derive(InputObject, Deserialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub description: String,
}
