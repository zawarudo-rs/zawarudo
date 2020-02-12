extern crate data_encoding;
extern crate ring;

use crate::storage::{schema::blocks, DbConn};
use chrono::prelude::NaiveDateTime;
use data_encoding::HEXUPPER;
use diesel::prelude::*;
use diesel::result::Error as DBError;
use diesel::RunQueryDsl;
use ring::digest::{Context, Digest, SHA256};
use serde_json::json;
use std::io::Read;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Block {
    pub index: Uuid,
    pub prev_index: Option<Uuid>,
    pub data: String,
    pub hash: String,
    pub prev_hash: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "blocks"]
pub struct CreateBlockForm<'a> {
    pub data: &'a str,
}

impl Block {
    pub fn new<'a>(db: &DbConn, new_block: CreateBlockForm) -> Result<Block, DBError> {
        use crate::storage::schema::blocks::dsl::*;
        let now = chrono::Local::now().naive_utc();

        let prev = blocks
            .order_by(created_at.desc())
            .first::<Block>(&**db)
            .optional()
            .unwrap_or(None);

        let (prev_id, _prev_hash) = match prev {
            Some(block) => (Some(block.index), Some(block.hash)),
            None => (None, None),
        };

        let curr_id = uuid::Uuid::new_v4();

        let json_data = json!({
            "index": curr_id.to_string(),
            "data": new_block.data,
            "prev_hash": _prev_hash,
            "created_at": now,
            "updated_at": now
        });

        let digest = sha256_digest(json_data.to_string().as_bytes());
        let hashed_data = HEXUPPER.encode(digest.as_ref());

        diesel::insert_into(blocks)
            .values((
                index.eq(uuid::Uuid::new_v4()),
                prev_index.eq(prev_id),
                data.eq(new_block.data),
                prev_hash.eq(_prev_hash.as_ref()),
                hash.eq(hashed_data),
                created_at.eq(now),
                updated_at.eq(now),
            ))
            .get_result::<Block>(&**db)
    }

    pub fn get_all(db: &DbConn, _limit: i64, _offset: i64) -> Result<Option<Vec<Block>>, DBError> {
        use crate::storage::schema::blocks::dsl::*;
        blocks
            .order_by(created_at.desc())
            .limit(_limit)
            .offset(_offset)
            .load::<Block>(&**db)
            .optional()
    }
}

fn sha256_digest<R: Read>(mut reader: R) -> Digest {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    context.finish()
}
