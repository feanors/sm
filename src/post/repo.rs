use crate::schema::posts::{self};
use crate::user;
use chrono::Utc;
use diesel::{
    prelude::Associations, ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable,
    Selectable, SelectableHelper,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Debug, Queryable, Selectable, Insertable, Identifiable, Associations)]
#[diesel(table_name = posts)]
#[diesel(belongs_to(user::repo::User, foreign_key = posted_by))]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Post {
    pub id: uuid::Uuid,
    pub posted_by: uuid::Uuid,
    pub content: String,
    pub created_at: chrono::DateTime<Utc>,
}

impl Post {
    fn from_domain_model(u: &super::model::Post) -> Post {
        Post {
            id: u.id.clone(),
            posted_by: u.posted_by.clone(),
            content: u.content.clone(),
            created_at: u.created_at.clone(),
        }
    }

    fn to_domain_model(self) -> super::model::Post {
        super::model::Post {
            id: self.id,
            posted_by: self.posted_by,
            content: self.content,
            created_at: self.created_at,
        }
    }
}

pub async fn create_post(
    conn: &mut AsyncPgConnection,
    p: &super::model::Post,
) -> Result<(), diesel::result::Error> {
    let db_model = Post::from_domain_model(p);
    diesel::insert_into(posts::table)
        .values(db_model)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn get_posts(
    conn: &mut AsyncPgConnection,
    user_id: uuid::Uuid,
) -> Result<Vec<super::model::Post>, diesel::result::Error> {
    let result = posts::table
        .filter(posts::posted_by.eq(user_id))
        .select(Post::as_select())
        .load(conn)
        .await?;
    Ok(result.into_iter().map(|p| p.to_domain_model()).collect())
}
