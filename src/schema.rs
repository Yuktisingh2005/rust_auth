// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        is_verified -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        is_active -> Bool,
    }
}
