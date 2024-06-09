use anyhow::Result;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::db::schema::forums;
use crate::db::schema::forums::{access_level, description, is_locked, title};

use super::user::AccessLevel;

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = forums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Forum {
    pub id: i32,

    pub title: String,
    pub description: String,

    pub is_locked: bool,

    pub access_level: AccessLevel,
}

impl Forum {
    pub async fn by_id(conn: &mut AsyncPgConnection, id: i32) -> Result<Self> {
        use crate::db::schema::forums::dsl::forums;

        let result = forums
            .find(id)
            .select(Self::as_select())
            .first(conn)
            .await?;

        Ok(result)
    }

    pub async fn by_title(conn: &mut AsyncPgConnection, value: &str) -> Result<Self> {
        use crate::db::schema::forums::dsl::{forums, title};

        let result = forums
            .filter(title.eq(value))
            .select(Self::as_select())
            .first(conn)
            .await?;

        Ok(result)
    }

    pub async fn by_description(conn: &mut AsyncPgConnection, value: &str) -> Result<Self> {
        use crate::db::schema::forums::dsl::{description, forums};

        let result = forums
            .filter(description.eq(value))
            .select(Self::as_select())
            .first(conn)
            .await?;

        Ok(result)
    }

    pub async fn by_access_level(
        conn: &mut AsyncPgConnection,
        value: &AccessLevel,
        limit: i64,
    ) -> Result<Vec<Self>> {
        use crate::db::schema::forums::dsl::{access_level, forums};

        let results = forums
            .filter(access_level.eq(value))
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }

    pub async fn by_title_like(
        conn: &mut AsyncPgConnection,
        value: &str,
        limit: i64,
    ) -> Result<Vec<Self>> {
        use crate::db::schema::forums::dsl::{forums, title};

        let results = forums
            .filter(title.like(value))
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }

    pub async fn by_description_like(
        conn: &mut AsyncPgConnection,
        value: &str,
        limit: i64,
    ) -> Result<Vec<Self>> {
        use crate::db::schema::forums::dsl::{description, forums};

        let results = forums
            .filter(description.like(value))
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }

    pub async fn by_is_locked(
        conn: &mut AsyncPgConnection,
        value: bool,
        limit: i64,
    ) -> Result<Vec<Self>> {
        use crate::db::schema::forums::dsl::{forums, is_locked};

        let results = forums
            .filter(is_locked.eq(value))
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }

    pub async fn all(conn: &mut AsyncPgConnection, limit: i64) -> Result<Vec<Self>> {
        use crate::db::schema::forums::dsl::forums;

        let results = forums
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> Result<()> {
        use crate::db::schema::forums::dsl::forums;

        diesel::delete(forums.find(id)).execute(conn).await?;

        Ok(())
    }
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = forums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct NewForum {
    pub title: String,
    pub description: String,

    pub is_locked: bool,

    pub access_level: AccessLevel,
}

impl NewForum {
    pub async fn insert(self, conn: &mut AsyncPgConnection) -> Result<Forum> {
        use crate::db::schema::forums::dsl::forums;

        let result = diesel::insert_into(forums)
            .values(&self)
            .returning(Forum::as_select())
            .get_result(conn)
            .await?;

        Ok(result)
    }

    pub async fn update(self, conn: &mut AsyncPgConnection, id: i32) -> Result<Forum> {
        use crate::db::schema::forums::dsl::forums;

        let result = diesel::update(forums.find(id))
            .set((
                title.eq(&self.title),
                description.eq(&self.description),
                is_locked.eq(&self.is_locked),
                access_level.eq(&self.access_level),
            ))
            .returning(Forum::as_select())
            .get_result(conn)
            .await?;

        Ok(result)
    }
}
