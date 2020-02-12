#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_json;

pub mod config;
pub mod graphql;
pub mod handler;
pub mod server;
pub mod storage;

use server::Server;

fn main() {
    config::init();
    Server::new().init().launch();
}
