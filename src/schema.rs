// @generated automatically by Diesel CLI.

diesel::table! {
    friendships (user1, user2) {
        user1 -> Uuid,
        user2 -> Uuid,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        posted_by -> Uuid,
        content -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        username -> Varchar,
        created_at -> Timestamptz,
        description -> Text,
    }
}

diesel::joinable!(posts -> users (posted_by));

diesel::allow_tables_to_appear_in_same_query!(
    friendships,
    posts,
    users,
);
