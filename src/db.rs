use actix_web::web::Data;
use bb8_postgres::{
    bb8::Pool,
    tokio_postgres::{Config, Error, NoTls},
    PostgresConnectionManager,
};
use std::str::FromStr;

pub type PgPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn init_pool(database_url: &str) -> Result<PgPool, Error> {
    let config = Config::from_str(database_url)?;
    let manager = PostgresConnectionManager::new(config, NoTls);
    Pool::builder().build(manager).await
}

pub async fn create_link(from: String, to: String, pool: Data<PgPool>) {
    let conn = pool.get().await.unwrap();
    conn.execute(
        r#"INSERT INTO links ("from", "to") VALUES ($1, $2);"#,
        &[&from, &to],
    )
    .await
    .unwrap();
}

pub async fn get_link(from: String, pool: Data<PgPool>) -> Option<String> {
    let conn = pool.get().await.unwrap();
    conn.query_one(r#"SELECT "to" FROM links WHERE "from"=$1;"#, &[&from])
        .await
        .ok()
        .map_or(None, |r| r.get(0))
}
