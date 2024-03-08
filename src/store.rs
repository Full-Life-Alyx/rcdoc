use redis::{aio::MultiplexedConnection, AsyncCommands, RedisError};
use sqlx::{types::Uuid, Pool, Postgres};

pub struct Store {
    pg: Pool<Postgres>,
    redis: MultiplexedConnection,
}

impl Store {
    pub fn new(pg: Pool<Postgres>, redis: MultiplexedConnection) -> Self {
        Self { pg, redis }
    }

    pub async fn get_doc_content(&self, doc_id: Uuid) -> Result<String, RedisError> {
        let mut redis = self.redis.clone();
        let res: Option<String> = redis.get(format!("build:text:{}", doc_id.simple())).await?;
        Ok(res.unwrap())
    }

    fn build_doc() {}
}
