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

    let user_a = transactional::<User, DataError, _>(move |txn| {
        Box::pin(async {
            let user = User::insert_accepts_conn(txn, InsertableUser {
                name: name_a
            }).await?;
            Ok(user)
        })
    }).await.expect("creates user");
    println!("User has name: {}", user_a.name);
    let found = User::get(name_a).await.expect("finds user");
    println!("Found user again: {}", found.name);

    let failure: Result<User, DataError> = transactional(move |txn| {
        Box::pin(async {
            let user = User::insert_accepts_conn(txn, InsertableUser {
                name: name_b
            }).await?;
            let found = User::get_accepts_conn(txn, name_b).await?;
            println!("Found user {}", found.name);
            Err(RollbackTransaction).map_err(DataError::from)
        })
    }).await;

    let found = User::get(name_b).await;
    match found {
        Ok(user) => println!("Should not hit this branch"),
        Err(err) => println!("Error finding: {:?}", err)
    }
}
