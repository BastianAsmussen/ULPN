use diesel::prelude::*;

use crate::db::schema::forums;
use super::user::AccessLevel;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = forums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Forum {
    pub id: i32,

    pub title: String,
    pub description: String,

    pub access_level: AccessLevel,
}

