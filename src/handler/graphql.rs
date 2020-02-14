use rocket::{response::content, State};

use crate::graphql::{GQLContext, Schema};
use crate::storage::DbConn;

#[rocket::post("/", data = "<request>")]
pub fn graphql(
    database: DbConn,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &GQLContext { database })
}

#[rocket::get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::playground_source("/graphql")
}
