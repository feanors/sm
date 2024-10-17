use diesel_async::{
    pooled_connection::deadpool::{Pool, PoolError},
    AsyncPgConnection,
};
use thiserror::Error;

pub struct LikeService {
    db_pool: Pool<AsyncPgConnection>,
    nats_client: async_nats::Client,
}

#[derive(Error, Debug)]
pub enum LikeServiceError {
    #[error("database error")]
    DBResult(#[from] diesel::result::Error),
    #[error("database driver error")]
    DBPool(#[from] PoolError),
    #[error("serde error")]
    Serde(#[from] serde_json::Error),
    #[error("nats publish error")]
    Nats(#[from] async_nats::PublishError),
}

use crate::user::model::UserDTO;

use super::{
    model::{CreateLikeDTO, LikeEvent},
    repo::{self},
};

impl LikeService {
    pub fn new(db_pool: Pool<AsyncPgConnection>, nats_client: async_nats::Client) -> LikeService {
        LikeService {
            db_pool,
            nats_client,
        }
    }

    pub async fn crate_like(&self, l: CreateLikeDTO) -> Result<(), LikeServiceError> {
        let mut conn = self.db_pool.get().await?;
        let serialized_like_event = serde_json::to_vec(&LikeEvent::new(&l))?;
        repo::create_like(&mut conn, l).await?;
        self.nats_client
            .publish("likes", serialized_like_event.into())
            .await?;
        Ok(())
    }

    pub async fn get_likes(&self, user_id: uuid::Uuid) -> Result<Vec<UserDTO>, LikeServiceError> {
        let mut conn = self.db_pool.get().await?;

        let r = repo::get_likes(&mut conn, user_id).await?;

        Ok(r.into_iter().map(|u| u.into()).collect())
    }
}
