use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::identities;

use anyhow::Result;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = identities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Identity {
    pub id: i32,

    pub value: String,
}

impl Identity {
    pub async fn by_id(conn: &mut AsyncPgConnection, id: i32) -> Result<Self> {
        use crate::db::schema::identities::dsl::identities;

        let result = identities
            .find(id)
            .select(Self::as_select())
            .first(conn)
            .await?;

        Ok(result)
    }

    pub async fn by_value(conn: &mut AsyncPgConnection, value: &str) -> Result<Self> {
        use crate::db::schema::identities::dsl::identities;

        let result = identities
            .filter(crate::db::schema::identities::dsl::value.eq(value))
            .select(Self::as_select())
            .first(conn)
            .await?;

        Ok(result)
    }

    pub async fn by_values(conn: &mut AsyncPgConnection, values: Vec<&str>) -> Result<Vec<Self>> {
        use crate::db::schema::identities::dsl::identities;

        let results = identities
            .filter(crate::db::schema::identities::dsl::value.eq_any(values))
            .select(Self::as_select())
            .load(conn)
            .await?;

        Ok(results)
    }
    
    pub async fn all(conn: &mut AsyncPgConnection, limit: i64) -> Result<Vec<Self>> {
        use crate::db::schema::identities::dsl::identities;

        let results = identities
            .select(Self::as_select())
            .limit(limit)
            .load(conn)
            .await?;

        Ok(results)
    }
}
