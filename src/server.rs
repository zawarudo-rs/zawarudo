use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;

pub struct Server {
    config: Config,
}

#[get("/")]
pub fn ping() -> String {
    "hello world".to_owned()
}

impl Server {
    pub fn new() -> Self {
        let config = Config::build(Environment::Development)
            .port(9000)
            .finalize()
            .unwrap();

        Server { config }
    }

    pub fn init(self) -> rocket::Rocket {
        rocket::custom(self.config).mount("/ping", routes![ping])
    }
}
