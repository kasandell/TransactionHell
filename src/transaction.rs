use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use diesel_async::AsyncConnection;
use diesel_async::scoped_futures::{ScopedBoxFuture, ScopedFutureExt};
use crate::error::DataError;
use crate::db::{connection, DbConnection};

#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct Transaction<'a> {
    conn: &'a mut DbConnection<'a>
}

impl <'a> Debug for Transaction<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transaction")
    }
}

impl <'a> Transaction<'a> {
    pub fn new(conn: &mut DbConnection) -> Self{
        Self {
            conn: conn
        }
    }
}

pub async fn transactional<'a, 'life0, 'async_trait, R, E, F>(
    f: F,
) -> Result<R, E> //Pin<Box<dyn Future<Output = Result<R, E>> + Send + 'async_trait>>
    where
        F: for<'r> FnOnce(&'r mut Transaction) -> ScopedBoxFuture<'a, 'r, Result<R, DataError>> + Send + 'a + 'async_trait,
        E: From<DataError> + 'a + 'async_trait, // was + Send
        R: Send + 'a + 'async_trait,
        'a: 'async_trait,
        'life0: 'async_trait,
{
    let mut conn = connection().await.map_err(|e| DataError::from(e))?;
    let result = conn.transaction(move |_conn| {
        let mut txn = Transaction::new(_conn);
        let result = f(&mut txn);
        Box::pin(async {
            let res = result.await;
            res
        })
    }.scope_boxed()).await?;
    Ok(result)
}