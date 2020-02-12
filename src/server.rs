use crate::config;
use crate::graphql::{Mutations, Query, Schema};
use crate::handler;
use crate::storage;

use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new() -> Self {
        let mut database_config = HashMap::new();
        let mut databases = HashMap::new();

        database_config.insert("url", Value::from(config::database_url()));
        databases.insert("postgres_db", Value::from(database_config));

        let config = Config::build(Environment::Development)
            .port(config::port().parse().unwrap())
            .extra("databases", databases)
            .finalize()
            .unwrap();

        Server { config }
    }

    pub fn init(self) -> rocket::Rocket {
        rocket::custom(self.config)
            .attach(storage::DbConn::fairing())
            .manage(Schema::new(Query, Mutations))
            .mount("/ping", routes![handler::ping::ping])
            .mount(
                "/graphql",
                routes![handler::graphql::graphql, handler::graphql::graphiql],
            )
    }
}
