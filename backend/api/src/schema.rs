// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "access_level"))]
    pub struct AccessLevel;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AccessLevel;

    users (id) {
        id -> Int8,
        #[max_length = 8]
        unilogin -> Bpchar,
        access_level -> AccessLevel,
        is_admin -> Bool,
    }
}
