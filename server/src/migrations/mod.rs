use std::env;

use refinery::{
    config::{Config, ConfigDbType},
    embed_migrations,
};
use url::Url;

embed_migrations!("./");

pub async fn migrate() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not supplied.");

    let url = Url::parse(&database_url).expect("Invalid DATABASE_URL");
    let db_user = url.username();
    let db_pass = url.password().unwrap_or("");
    let db_host = url.host_str().unwrap_or("");
    let db_port = url.port().unwrap_or(5432).to_string();
    let db_name = url
        .path_segments()
        .and_then(|segments| segments.last())
        .unwrap_or("");

    let mut conn = Config::new(ConfigDbType::Postgres)
        .set_db_user(db_user)
        .set_db_pass(db_pass)
        .set_db_host(db_host)
        .set_db_port(&db_port)
        .set_db_name(db_name);
    println!("Running migrations");
    migrations::runner()
        .run(&mut conn)
        .expect("Running migrations failed");
}
