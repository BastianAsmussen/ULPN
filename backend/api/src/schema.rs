// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "access_level"))]
    pub struct AccessLevel;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AccessLevel;

    forums (id) {
        id -> Int4,
        #[max_length = 256]
        name -> Varchar,
        access_level -> AccessLevel,
    }
}

diesel::table! {
    identities (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    messages (id) {
        id -> Int8,
        forum_id -> Int4,
        sender_id -> Int4,
        reply_id -> Int8,
        identity -> Int4,
        contents -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AccessLevel;

    users (id) {
        id -> Int4,
        #[max_length = 8]
        unilogin -> Bpchar,
        #[max_length = 1024]
        full_name -> Varchar,
        access_level -> AccessLevel,
        is_admin -> Bool,
    }
}

diesel::joinable!(messages -> forums (forum_id));
diesel::joinable!(messages -> identities (identity));
diesel::joinable!(messages -> users (sender_id));

diesel::allow_tables_to_appear_in_same_query!(
    forums,
    identities,
    messages,
    users,
);
