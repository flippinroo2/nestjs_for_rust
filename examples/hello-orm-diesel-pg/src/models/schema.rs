// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
