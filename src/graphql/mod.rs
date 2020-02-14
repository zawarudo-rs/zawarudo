use crate::storage::{
    domain::block::{Block, CreateBlockForm},
    DbConn,
};
use diesel::result::Error as DBError;
use juniper::{graphql_value, Context, FieldError, IntoFieldError, RootNode};

pub struct GQLContext {
    pub database: DbConn,
}
enum CustomError {
    database(DBError),
}

impl IntoFieldError for CustomError {
    fn into_field_error(self) -> FieldError {
        let (message, error_type) = match self {
            CustomError::database(dberr) => match dberr {
                DBError::AlreadyInTransaction => {
                    ("Already in transaction", "DBAlreadyInTransaction")
                }
                DBError::NotFound => ("Data not found", "DBNotFound"),
                _ => ("db error", "DBUnindentifiedError"),
            },
            _ => ("error", "UnidentifiedError"),
        };
        FieldError::new(message, graphql_value!({ "type": error_type }))
    }
}

impl Context for GQLContext {}

pub struct Query;
pub struct Mutations;
pub type Schema = RootNode<'static, Query, Mutations>;

#[juniper::object(
    Context = GQLContext,
    Scalar = juniper::DefaultScalarValue,
)]
impl Block {
    pub fn index(&self) -> uuid::Uuid {
        self.index
    }

    pub fn hash(&self) -> String {
        self.hash.clone()
    }

    pub fn prev_hash(&self) -> Option<String> {
        self.prev_hash.clone()
    }

    pub fn data(&self) -> String {
        self.data.clone()
    }

    pub fn created_at(&self) -> chrono::NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> chrono::NaiveDateTime {
        self.updated_at
    }

    pub fn deleted_at(&self) -> Option<chrono::NaiveDateTime> {
        self.deleted_at
    }

    pub fn prev(&self, ctx: &GQLContext) -> Result<Option<Block>, CustomError> {
        match self.prev_index {
            None => Ok(None),
            Some(index) => match Self::get_by_id(&ctx.database, index) {
                Ok(data) => Ok(data),
                Err(err) => Err(CustomError::database(err)),
            },
        }
    }
}

type BlockList = Vec<Block>;

#[juniper::object(
    Context = GQLContext,
    Scalar = juniper::DefaultScalarValue
)]
impl Query {
    fn hello_world(ctx: &GQLContext) -> String {
        "Hello World".to_owned()
    }

    fn get_all_block(
        ctx: &GQLContext,
        limit: i32,
        offset: i32,
    ) -> Result<Option<BlockList>, CustomError> {
        match Block::get_all(&ctx.database, limit as i64, offset as i64) {
            Ok(data) => Ok(data),
            Err(err) => Err(CustomError::database(err)),
        }
    }

    fn verify_all_block(ctx: &GQLContext) -> Result<bool, CustomError> {
        match Block::verify(&ctx.database) {
            Ok(res) => Ok(res),
            Err(err) => Err(CustomError::database(err)),
        }
    }
}

#[juniper::object(
    Context = GQLContext,
    Scalar = juniper::DefaultScalarValue
)]
impl Mutations {
    pub fn create_new_block(ctx: &GQLContext, data: String) -> Result<Block, CustomError> {
        match Block::new(
            &ctx.database,
            CreateBlockForm {
                data: data.as_ref(),
            },
        ) {
            Ok(data) => Ok(data),
            Err(err) => Err(CustomError::database(err)),
        }
    }
}
