use diesel::prelude::*;

use crate::db::schema::identities;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = identities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Identity {
    pub id: i32,

    pub value: String,
}

