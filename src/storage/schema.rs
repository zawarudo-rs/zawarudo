table! {
    blocks (index) {
        index -> Uuid,
        prev_index -> Nullable<Uuid>,
        data -> Text,
        hash -> Text,
        prev_hash -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}
