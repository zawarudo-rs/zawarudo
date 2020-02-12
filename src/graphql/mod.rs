use crate::storage::DbConn;
use juniper::{Context, RootNode};

pub struct GQLContext {
    pub database: DbConn,
}

impl Context for GQLContext {}

pub struct Query;
pub struct Mutations;
pub type Schema = RootNode<'static, Query, Mutations>;

#[juniper::object(
    Context = GQLContext,
    Scalar = juniper::DefaultScalarValue
)]
impl Query {
    fn hello_world(ctx: &GQLContext) -> String {
        "Hello World".to_owned()
    }
}

#[juniper::object(
    Context = GQLContext,
    Scalar = juniper::DefaultScalarValue
)]
impl Mutations {
    fn hello_world(ctx: &GQLContext) -> String {
        "Hello World".to_owned()
    }
}
