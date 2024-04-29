use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::error::{MyError, MyResult};
use std::env;

pub async fn create_pool() -> MyResult<Pool<Postgres>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not supplied.");

    match PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
    {
        Ok(pool) => Ok(pool),
        Err(e) => Err(MyError::PoolCreationFail {
            error: e.to_string(),
        }),
    }
}
