use std::time::Duration;
use bb8::PooledConnection;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::{Pool, RunError};
use tokio::sync::OnceCell;
static DB_URL: &str = "postgres://user:password@localhost:5433/db";
static POOL: OnceCell<Pool<AsyncPgConnection>> = OnceCell::const_new();

pub type ConnManage = AsyncDieselConnectionManager<AsyncPgConnection>;
pub type DbConnection<'a> = PooledConnection<'a, ConnManage>;
pub type ConnResult<'a> = Result<DbConnection<'a>, RunError>;
type DB = diesel::pg::Pg;

async fn init_db() -> Pool<AsyncPgConnection>{
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(DB_URL);
    let pool_size = 2;
    let mut builder = Pool::builder();
    log::info!("Initializing connection pool with {} connections", pool_size);
    builder.max_size(pool_size).connection_timeout(Duration::from_secs(2)).build(manager).await.expect("Failed to create db pool")
}


pub async fn connection<'a>() -> ConnResult<'a> {
    POOL.get_or_init(init_db).await.get().await
}