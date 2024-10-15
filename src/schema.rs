// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        #[max_length = 32]
        id -> Varchar,
        #[max_length = 100]
        username -> Varchar,
        created_at -> Timestamptz,
        description -> Text,
    }
}
