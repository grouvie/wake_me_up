#[derive(sqlx::FromRow)]
pub(crate) struct UserDB {
    pub(crate) id: i32,
    #[allow(dead_code)]
    pub(crate) username: String,
    pub(crate) password_hash: String,
}

#[derive(sqlx::FromRow)]
pub(crate) struct DeviceDB {
    pub(crate) id: i32,
    #[allow(dead_code)]
    pub(crate) user_id: i32,
    pub(crate) name: String,
    pub(crate) mac_address: String,
}
