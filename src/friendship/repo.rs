use crate::schema::friendships::{user1, user2};
use crate::schema::users::{self, id};
use crate::user;
use crate::{schema::friendships, user::model::User};
use diesel::{
    prelude::Associations, ExpressionMethods, Identifiable, Insertable, QueryDsl, Queryable,
    Selectable, SelectableHelper,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use super::model::AddFriendDTO;

#[derive(Debug, Queryable, Selectable, Insertable, Identifiable, Associations)]
#[diesel(table_name = friendships)]
#[diesel(belongs_to(User, foreign_key = user1))]
#[diesel(primary_key(user1, user2))]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Friendship {
    pub user1: uuid::Uuid,
    pub user2: uuid::Uuid,
}

impl From<AddFriendDTO> for Friendship {
    fn from(u: AddFriendDTO) -> Self {
        let (u1, u2) = if u.user1 > u.user2 {
            (u.user2, u.user1)
        } else {
            (u.user1, u.user2)
        };
        Friendship {
            user1: u1,
            user2: u2,
        }
    }
}

pub async fn add_friend(
    conn: &mut AsyncPgConnection,
    f: AddFriendDTO,
) -> Result<(), diesel::result::Error> {
    let db_model: Friendship = f.into();
    diesel::insert_into(friendships::table)
        .values(db_model)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn get_friends(
    conn: &mut AsyncPgConnection,
    user_id: uuid::Uuid,
) -> Result<Vec<user::model::User>, diesel::result::Error> {
    let f = friendships::table
        .filter(user1.eq(user_id))
        .or_filter(user2.eq(user_id))
        .select(Friendship::as_select())
        .load(conn)
        .await?;

    let friend_ids: Vec<uuid::Uuid> = f
        .into_iter()
        .map(|f| if f.user1 == user_id { f.user2 } else { f.user1 })
        .collect();

    let result = users::table
        .filter(id.eq_any(friend_ids))
        .select(user::repo::User::as_select())
        .load(conn)
        .await?;
    Ok(result.into_iter().map(|u| u.into()).collect())
}
