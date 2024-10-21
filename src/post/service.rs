use async_graphql::Context;
use diesel_async::{
    pooled_connection::deadpool::{Pool, PoolError},
    AsyncPgConnection,
};
use thiserror::Error;
use tokio::try_join;
use crate::graphql::post::CreatePostDTO;
use super::{
    model::{Post},
    repo,
};

pub struct PostService {
    db_pool: Pool<AsyncPgConnection>,
}

#[derive(Error, Debug)]
pub enum PostServiceError {
    #[error("database error")]
    DBResult(#[from] diesel::result::Error),
    #[error("database driver error")]
    DBPool(#[from] PoolError),
}

use crate::user;

impl PostService {
    pub fn new(db_pool: Pool<AsyncPgConnection>) -> PostService {
        PostService { db_pool }
    }

    pub async fn create_post(&self,  p: CreatePostDTO) -> Result<Post, PostServiceError> {
        let mut conn = self.db_pool.get().await?;
        let post: Post = p.into();
        repo::create_post(&mut conn, post.clone()).await?;
        Ok(post)
    }

    pub async fn get_posts(&self, user_id: uuid::Uuid) -> Result<Vec<Post>, PostServiceError> {
        let mut conn = self.db_pool.get().await?;
        let posts =             repo::get_posts(&mut conn, user_id).await?;
        Ok(posts)
    }

    pub async fn get_post(&self, post_id: uuid::Uuid) -> Result<Post, PostServiceError> {
        let mut conn = self.db_pool.get().await?;
        let post = repo::get_post(&mut conn, post_id).await?;
        Ok(post)
    }
}
