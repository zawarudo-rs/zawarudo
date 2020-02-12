-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "blocks" (
    "index" UUID PRIMARY KEY,
    "prev_index" UUID,
    "data" TEXT NOT NULL,
    "hash" TEXT NOT NULL,
    "prev_hash" TEXT,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP NOT NULL,
    "deleted_at" TIMESTAMP
);
