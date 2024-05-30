use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use diesel_async::AsyncConnection;
use diesel_async::scoped_futures::{ScopedBoxFuture, ScopedFutureExt};
use futures::future::BoxFuture;
use tokio::runtime::Handle;
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
    pub fn new(conn: &'a mut DbConnection<'a>) -> Self{
        Self {
            conn: conn
        }
    }
}

pub async fn transactional<T, E, F>(f: F) -> Result<T, E>
    where
        T: Send + 'static,
        E: From<DataError>,
        F: for<'conn> FnOnce(&'conn mut DbConnection) -> BoxFuture<'conn, Result<T, DataError>> + Send + 'static,
{

    let mut conn = connection().await.map_err(|e| DataError::from(e))?;
    let result = conn.transaction(|mut _conn| async move {
        //let mut txn = Transaction::new(_conn);
            //let mut txn = Transaction::new(_conn);
        f(&mut _conn).await
    }.scope_boxed()).await?;
    Ok(result)

}
/*
let mut pool = pool.get().map_err(|err| TxError::Other(Box::new(err)))?;

let thread_result: Result<Result<T, TxError>, tauri::Error> = tauri::async_runtime::spawn_blocking(move || {
    let handle = tauri::async_runtime::TokioHandle::current();
    let transaction_result: Result<T, TxError> = pool.transaction(|conn| {
        let result: Result<T, Box<dyn Error>> = handle.block_on(async {
            f(conn).await
        });
        result.map_err(|err| TxError::Other(err))
    });
    transaction_result
}).await;
 */

/*
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

 */