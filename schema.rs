table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        is_verified -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        is_active -> Bool,
    }
}
