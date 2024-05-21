use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::db::schema::messages;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: i64,

    pub forum_id: i32,
    pub sender_id: i32,

    pub reply_id: i64,

    pub identity_id: i32,
    pub contents: String,
    pub is_published: bool,

    pub created_at: DateTime<Utc>,
}
