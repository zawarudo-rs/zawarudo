extern crate diesel;
pub mod domain;
pub mod schema;

use diesel::pg::PgConnection;

#[database("postgres_db")]
pub struct DbConn(PgConnection);
