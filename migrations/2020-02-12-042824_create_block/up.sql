-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "blocks" (
    "index" BIGINT PRIMARY KEY,
    "data" TEXT NOT NULL,
    "hash" TEXT NOT NULL,
    "prev_hash" TEXT NOT NULL,
    "created_at" TIMESTAMP NOT NULL,
    "update_at" TIMESTAMP NOT NULL,
    "deleted_at" TIMESTAMP
);
