use diesel_async::{
    pooled_connection::deadpool::{Pool, PoolError},
    AsyncPgConnection,
};
use thiserror::Error;
use tokio::try_join;

use super::{
    model::{CreatePostDTO, Post, PostDTO, PostsDTO},
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

    pub async fn create_post(&self, p: CreatePostDTO) -> Result<PostDTO, PostServiceError> {
        let mut conn = self.db_pool.get().await?;
        let post: Post = p.into();
        repo::create_post(&mut conn, post.clone()).await?;
        Ok(post.into())
    }

    pub async fn get_posts(&self, user_id: uuid::Uuid) -> Result<PostsDTO, PostServiceError> {
        let mut conns = try_join!(self.db_pool.get(), self.db_pool.get())?;

        let res = try_join!(
            user::repo::get_user(&mut conns.0, user_id),
            repo::get_posts(&mut conns.1, user_id)
        )?;

        Ok(PostsDTO::new(
            res.0.into(),
            res.1.into_iter().map(|p| p.into()).collect(),
        ))
    }
}
