use sqlx::{Connection, Error as SqlxError, PgConnection};
use deadpool::managed::{ Manager, RecycleResult };
use async_trait::async_trait;

pub struct PoolManager {
  pub url: String,
}
pub type Pool = deadpool::managed::Pool<PoolManager>;
// pub struct DbPool {}

/*
impl DbPool {
  pub fn new(url: String, size: usize) -> Pool {
    Pool::new(PoolManager { url }, size)
  }
}
*/

#[async_trait]
impl Manager for PoolManager {
    type Type = PgConnection;
    type Error = SqlxError;
    async fn create(&self) -> Result<PgConnection, SqlxError> {
        PgConnection::connect(&self.url).await
    }
    async fn recycle(&self, obj: &mut PgConnection) -> RecycleResult<SqlxError> {
        Ok(obj.ping().await?)
    }

    fn detach(&self, _obj: &mut Self::Type) {}
}
