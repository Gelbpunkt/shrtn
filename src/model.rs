use crate::schema::{links, links::dsl::*};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Debug, Insertable)]
#[table_name = "links"]
pub struct NewLink {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Link {
    pub id: i32,
    pub from: String,
    pub to: String,
}

impl Link {
    pub fn insert(link: NewLink, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(links).values(&link).execute(conn)
    }

    pub fn get(url: String, conn: &PgConnection) -> QueryResult<Link> {
        println!("{}", url);
        links.filter(from.eq(url)).first::<Link>(conn)
    }
}
