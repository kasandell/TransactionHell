use crate::db;
use crate::schema::users;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use crate::db::DbConnection;
use crate::error::DataError;
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable, Clone, PartialEq)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct InsertableUser<'a> {
    // could be more fields
    pub name: &'a str
}

impl User {
    pub async fn insert<'a>(transaction: &mut Transaction<'_>, user: InsertableUser<'a>) -> Result<User, DataError>{
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(transaction)
            .await?;
        Ok(user)
    }

    pub async fn insert_accepts_conn<'a>(transaction: &mut DbConnection<'_>, user: InsertableUser<'a>) -> Result<User, DataError>{
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(transaction)
            .await?;
        Ok(user)
    }

    pub async fn get<'a>(name: &'a str) -> Result<User, DataError>{
        let mut conn = db::connection().await?;
        let user = users::table.filter(users::name.eq(name))
            .first::<User>(&mut conn)
            .await?;
        Ok(user)
    }

    pub async fn get_accepts_conn<'a>(transaction: &mut DbConnection<'_>, name: &'a str) -> Result<User, DataError>{
        let user = users::table.filter(users::name.eq(name))
            .first::<User>(transaction)
            .await?;
        Ok(user)
    }

    pub async fn transactional_get<'a>(transaction: &mut Transaction<'_>, name: &'a str) -> Result<User, DataError>{
        let user = users::table.filter(users::name.eq(name))
            .first::<User>(transaction)
            .await?;
        Ok(user)
    }
}
