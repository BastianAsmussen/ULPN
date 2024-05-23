use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::identities;

use anyhow::Result;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = identities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Identity {
    pub id: i32,

    pub owner_id: i32,
    pub forum_id: i32,

    pub name: String,
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

    pub async fn by_owner_id(conn: &mut AsyncPgConnection, owner_id: i32) -> Result<Vec<Self>> {
        use crate::db::schema::identities::dsl::identities;

        let result = identities
            .filter(crate::db::schema::identities::owner_id.eq(owner_id))
            .select(Self::as_select())
            .load(conn)
            .await?;

        Ok(result)
    }

    pub async fn by_forum_id(conn: &mut AsyncPgConnection, forum_id: i32) -> Result<Vec<Self>> {
        use crate::db::schema::identities::dsl::identities;

        let result = identities
            .filter(crate::db::schema::identities::forum_id.eq(forum_id))
            .select(Self::as_select())
            .load(conn)
            .await?;

        Ok(result)
    }
}
