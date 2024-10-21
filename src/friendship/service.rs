use async_graphql::Context;
use diesel_async::{
    pooled_connection::deadpool::{Pool, PoolError},
    AsyncPgConnection,
};
use thiserror::Error;
use crate::graphql::friend::AddFriendDTO;
use crate::user;

pub struct FriendshipService {
    db_pool: Pool<AsyncPgConnection>,
}

#[derive(Error, Debug)]
pub enum FriendshipServiceError {
    #[error("database error")]
    DBResult(#[from] diesel::result::Error),
    #[error("database driver error")]
    DBPool(#[from] PoolError),
}


use super::{ repo};

impl FriendshipService {
    pub fn new(db_pool: Pool<AsyncPgConnection>) -> FriendshipService {
        FriendshipService { db_pool }
    }

    pub async fn add_friend(&self, f: AddFriendDTO) -> Result<(), FriendshipServiceError> {
        let mut conn = self.db_pool.get().await?;
        repo::add_friend(&mut conn, f).await?;
        Ok(())
    }

    pub async fn get_friends(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Vec<user::model::User>, FriendshipServiceError> {
        let mut conn = self.db_pool.get().await?;

        let friends = repo::get_friends(&mut conn, user_id).await?;

        Ok(friends)
    }
}
