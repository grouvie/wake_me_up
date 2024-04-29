use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct Client {
    pub pool: Pool<Postgres>,
}

impl Client {
    pub(crate) async fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}
