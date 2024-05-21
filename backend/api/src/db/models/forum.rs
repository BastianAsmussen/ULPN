use diesel::prelude::*;

use super::user::AccessLevel;
use crate::db::schema::forums;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = forums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Forum {
    pub id: i32,

    pub title: String,
    pub description: String,

    pub access_level: AccessLevel,
}

