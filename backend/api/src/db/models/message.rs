use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::db::schema::messages;

use anyhow::Result;

#[derive(Debug, Serialize, Queryable, Selectable, Identifiable)]
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

impl Message {
    pub async fn by_id(conn: &mut AsyncPgConnection, id: i64) -> Result<Self> {
        use crate::db::schema::messages::dsl::messages;

        let result = messages
            .find(id)
            .select(Self::as_select())
            .first(conn)
            .await?;

        Ok(result)
    }

    pub async fn by_forum_id(
        conn: &mut AsyncPgConnection,
        forum_id: i32,
        limit: i64,
    ) -> Result<Vec<Self>> {
        use crate::db::schema::messages::dsl::messages;

        let results = messages
            .filter(crate::db::schema::messages::dsl::forum_id.eq(forum_id))
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }

    pub async fn by_sender_id(
        conn: &mut AsyncPgConnection,
        sender_id: i32,
        limit: i64,
    ) -> Result<Vec<Self>> {
        use crate::db::schema::messages::dsl::messages;

        let results = messages
            .filter(crate::db::schema::messages::dsl::sender_id.eq(sender_id))
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }

    pub async fn by_reply_id(
        conn: &mut AsyncPgConnection,
        reply_id: i64,
        limit: i64,
    ) -> Result<Vec<Self>> {
        use crate::db::schema::messages::dsl::messages;

        let results = messages
            .filter(crate::db::schema::messages::dsl::reply_id.eq(reply_id))
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }

    pub async fn by_identity_id(
        conn: &mut AsyncPgConnection,
        identity_id: i32,
        limit: i64,
    ) -> Result<Vec<Self>> {
        use crate::db::schema::messages::dsl::messages;

        let results = messages
            .filter(crate::db::schema::messages::dsl::identity_id.eq(identity_id))
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }

    pub async fn by_contents(
        conn: &mut AsyncPgConnection,
        contents: &str,
        limit: i64,
    ) -> Result<Vec<Self>> {
        use crate::db::schema::messages::dsl::messages;

        let results = messages
            .filter(crate::db::schema::messages::dsl::contents.like(contents))
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }

    pub async fn by_is_published(
        conn: &mut AsyncPgConnection,
        is_published: bool,
        limit: i64,
    ) -> Result<Vec<Self>> {
        use crate::db::schema::messages::dsl::messages;

        let results = messages
            .filter(crate::db::schema::messages::dsl::is_published.eq(is_published))
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }

    pub async fn all(conn: &mut AsyncPgConnection, limit: i64) -> Result<Vec<Self>> {
        use crate::db::schema::messages::dsl::messages;

        let results = messages
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct NewMessage {
    pub forum_id: i32,
    pub sender_id: i32,

    pub reply_id: Option<i64>,
    pub identity_id: i32,

    pub contents: String,
    pub is_published: bool,
}

impl NewMessage {
    pub async fn insert(self, conn: &mut AsyncPgConnection) -> Result<Message> {
        use crate::db::schema::messages::dsl::messages;

        let result = diesel::insert_into(messages)
            .values(&self)
            .returning(Message::as_select())
            .get_result(conn)
            .await?;

        Ok(result)
    }
}
