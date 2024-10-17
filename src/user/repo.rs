use chrono::Utc;
use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, Identifiable, Insertable, Queryable, Selectable, SelectableHelper,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::schema::users::{self, *};

#[derive(Debug, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub description: String,
    pub created_at: chrono::DateTime<Utc>,
}

impl From<super::model::User> for User {
    fn from(u: super::model::User) -> Self {
        User {
            id: u.id,
            username: u.username,
            description: u.description,
            created_at: u.created_at,
        }
    }
}

impl Into<super::model::User> for User {
    fn into(self) -> super::model::User {
        super::model::User {
            id: self.id,
            username: self.username,
            description: self.description,
            created_at: self.created_at,
        }
    }
}

pub async fn create_user(
    conn: &mut AsyncPgConnection,
    u: super::model::User,
) -> Result<(), diesel::result::Error> {
    let db_model: User = u.into();
    diesel::insert_into(users::table)
        .values(db_model)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn get_user(
    conn: &mut AsyncPgConnection,
    user_id: uuid::Uuid,
) -> Result<super::model::User, diesel::result::Error> {
    let result = users::table
        .filter(id.eq(user_id))
        .select(User::as_select())
        .first(conn)
        .await?;
    Ok(result.into())
}
