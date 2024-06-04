use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::users;

use anyhow::Result;

use serde::{Deserialize, Serialize};

#[derive(Debug, diesel_derive_enum::DbEnum, Serialize, Deserialize, Clone)]
#[ExistingTypePath = "crate::db::schema::sql_types::AccessLevel"]
pub enum AccessLevel {
    Child,
    Parent,
    Professional,
    Administrator,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,

    pub full_name: String,
    pub email: String,

    pub access_level: AccessLevel,
}

impl User {
    pub async fn by_id(conn: &mut AsyncPgConnection, id: i32) -> Result<Self> {
        use crate::db::schema::users::dsl::users;

        let result = users.find(id).select(Self::as_select()).first(conn).await?;

        Ok(result)
    }

    pub async fn by_email(conn: &mut AsyncPgConnection, value: &str) -> Result<Self> {
        use crate::db::schema::users::dsl::{email, users};

        let result = users
            .filter(email.eq(value))
            .select(Self::as_select())
            .first(conn)
            .await?;

        Ok(result)
    }

    pub async fn by_name(
        conn: &mut AsyncPgConnection,
        value: &str,
        limit: i64,
    ) -> Result<Vec<Self>> {
        use crate::db::schema::users::dsl::{full_name, users};

        let results = users
            .filter(full_name.like(value))
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }

    pub async fn by_access_level(
        conn: &mut AsyncPgConnection,
        value: &AccessLevel,
        limit: i64,
    ) -> Result<Vec<Self>> {
        use crate::db::schema::users::dsl::{access_level, users};

        let results = users
            .filter(access_level.eq(value))
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }

    pub async fn all(conn: &mut AsyncPgConnection, limit: i64) -> Result<Vec<Self>> {
        use crate::db::schema::users::dsl::users;

        let results = users
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub full_name: String,
    pub email: String,
    pub access_level: AccessLevel,
}

impl NewUser {
    pub async fn insert(self, conn: &mut AsyncPgConnection) -> Result<User> {
        use crate::db::schema::users::dsl::users;

        let result = diesel::insert_into(users)
            .values(&self)
            .returning(User::as_select())
            .get_result(conn)
            .await?;

        Ok(result)
    }
}
