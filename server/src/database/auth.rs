use crate::database::data::client::Client;
use crate::error::{MyError, MyResult};

use super::data::models::UserDB;

impl Client {
    pub(crate) async fn get_user(&mut self, username: String) -> MyResult<UserDB> {
        let stream = sqlx::query_as::<_, UserDB>("SELECT * FROM public.user WHERE username = $1")
            .bind(username)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => MyError::DatabaseRowNotFound {
                    error: e.to_string(),
                },
                sqlx::Error::ColumnNotFound(_) => MyError::DatabaseRowNotFound {
                    error: e.to_string(),
                },
                _ => MyError::Database {
                    error: e.to_string(),
                },
            })?;

        Ok(stream)
    }
}
