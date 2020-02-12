#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod server;

use server::Server;

fn main() {
    Server::new().init().launch();
}
