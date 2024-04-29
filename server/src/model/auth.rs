use super::controller::ModelController;
use crate::proto::login_request::LoginRequestProto;
use crate::{
    database::data::client::Client,
    error::{MyError, MyResult},
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};

impl ModelController {
    pub(crate) async fn check_login(&self, login: LoginRequestProto) -> MyResult<i32> {
        let mut client = Client::new(self.client.pool.clone()).await;

        let user = client.get_user(login.username).await?;

        let argon2 = Argon2::default();

        let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| MyError::LoginFail)?;

        argon2
            .verify_password(login.password.as_bytes(), &parsed_hash)
            .map_err(|_| MyError::LoginFail)
            .map(|_| user.id)
    }
}
