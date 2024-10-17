use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, Identifiable, Insertable, Queryable, Selectable, SelectableHelper,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    schema::{
        likes::{self, liked_post},
        users::{self, *},
    },
    user,
};

use super::model::CreateLikeDTO;

#[derive(Debug, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = likes)]
#[diesel(primary_key(liked_by, liked_post))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Like {
    pub liked_by: uuid::Uuid,
    pub liked_post: uuid::Uuid,
}

impl From<CreateLikeDTO> for Like {
    fn from(l: CreateLikeDTO) -> Self {
        Like {
            liked_by: l.liked_by,
            liked_post: l.liked_post,
        }
    }
}

pub async fn create_like(
    conn: &mut AsyncPgConnection,
    l: super::model::CreateLikeDTO,
) -> Result<(), diesel::result::Error> {
    let db_model: Like = l.into();
    diesel::insert_into(likes::table)
        .values(db_model)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn get_likes(
    conn: &mut AsyncPgConnection,
    post_id: uuid::Uuid,
) -> Result<Vec<user::model::User>, diesel::result::Error> {
    let user_ids = likes::table
        .filter(liked_post.eq(post_id))
        .select(Like::as_select())
        .load(conn)
        .await?;

    let user_ids: Vec<uuid::Uuid> = user_ids.into_iter().map(|l| l.liked_by).collect();

    let result = users::table
        .filter(id.eq_any(user_ids))
        .select(user::repo::User::as_select())
        .load(conn)
        .await?;
    Ok(result.into_iter().map(|u| u.into()).collect())
}
