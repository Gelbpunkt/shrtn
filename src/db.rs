use crate::model::{Link, NewLink};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use std::ops::Deref;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}

pub fn create_link(from: String, to: String, pool: &PgPool) -> Result<(), &'static str> {
    let new_link = NewLink { from: from, to: to };
    Link::insert(new_link, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Can't insert link")
}

pub fn get_link(from: String, pool: &PgPool) -> Result<Link, diesel::result::Error> {
    Link::get(from, get_conn(pool).unwrap().deref()) //.map_err(|_| "Can't select")
}
