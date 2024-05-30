use diesel::result::DatabaseErrorKind;
use diesel::result::Error::RollbackTransaction;
use diesel_async::scoped_futures::ScopedFutureExt;
use crate::entity::{InsertableUser, User};
use crate::error::DataError;
use crate::transaction::transactional;

mod db;
mod entity;
mod schema;
mod error;
mod transaction;


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let name_a = "Kyle";
    let name_b = "Not Kyle";
    let user_a = transactional::<User, DataError, _>(|txn| async move {
        let user = User::insert(txn, InsertableUser {
            name: name_a
        }).await?;
        Ok(user)
    }.scope_boxed()).await.expect("creates user");
    println!("User has name: {}", user_a.name);
    let found = User::get(name_a).await.expect("finds user");
    println!("Found user again: {}", found.name);

    let failure: Result<User, DataError> = transactional(|txn| async move {
        let user = User::insert(txn, InsertableUser {
            name: name_b
        }).await?;
        let found = User::get(name_b).await?;
        println!("Found user {}", found.name);
        Err(RollbackTransaction).map_err(DataError::from)
    }.scope_boxed()).await;

    let found = User::get(name_b).await;
    match found {
        Ok(user) => println!("Should not hit this branch"),
        Err(err) => println!("Error finding: {:?}", err)
    }
}
