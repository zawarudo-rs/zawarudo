#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod config;
pub mod handler;
pub mod server;
pub mod storage;

use server::Server;

fn main() {
    config::init();
    Server::new().init().launch();
}
