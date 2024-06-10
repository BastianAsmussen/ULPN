use diesel::{Identifiable, Queryable, QueryDsl, Selectable, SelectableHelper, ExpressionMethods};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::db::schema::settings;
use crate::db::schema::settings::{key, value};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Eq, PartialEq)]
#[diesel(table_name = settings, primary_key(key))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Settings {
    pub key: String,
    pub value: String,
}

impl Settings {
    pub async fn by_key(conn: &mut AsyncPgConnection, key_: &str) -> Result<Self, diesel::result::Error> {
        use crate::db::schema::settings::dsl::settings;
        
        let result = settings
            .find(key_)
            .select(Self::as_select())
            .first(conn)
            .await?;
        
        Ok(result)
    }
    
    pub async fn all(conn: &mut AsyncPgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::db::schema::settings::dsl::settings;

        let results = settings
            .select(Self::as_select())
            .load(conn)
            .await?;
        
        Ok(results)
    }
    
    pub async fn insert(conn: &mut AsyncPgConnection, key_: &str, value_: &str) -> Result<usize, diesel::result::Error> {
        use crate::db::schema::settings::dsl::settings;

        let result = diesel::insert_into(settings)
            .values((key.eq(key_), value.eq(value_)))
            .execute(conn)
            .await?;
        
        Ok(result)
    }
    
    pub async fn update(conn: &mut AsyncPgConnection, key_: &str, value_: &str) -> Result<usize, diesel::result::Error> {
        use crate::db::schema::settings::dsl::settings;

        let result = diesel::update(settings.find(key_))
            .set(value.eq(value_))
            .execute(conn)
            .await?;
        
        Ok(result)
    }
    
    pub async fn delete(conn: &mut AsyncPgConnection, key_: &str) -> Result<usize, diesel::result::Error> {
        use crate::db::schema::settings::dsl::settings;

        let result = diesel::delete(settings.find(key_))
            .execute(conn)
            .await?;
        
        Ok(result)
    }
}
