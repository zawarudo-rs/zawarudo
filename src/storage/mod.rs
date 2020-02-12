extern crate diesel;
pub mod schema;

use diesel::pg::PgConnection;

#[database("postgres_db")]
pub struct DbConn(PgConnection);
