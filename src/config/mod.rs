extern crate dotenv;

use std::env;

mod default;

pub fn init() {
    dotenv::dotenv();
}

pub fn port() -> String {
    env::var("PORT").unwrap_or(default::port())
}

pub fn database_url() -> String {
    env::var("DATABASE_URL").unwrap_or(default::database_url())
}
