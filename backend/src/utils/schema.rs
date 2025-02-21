// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        hashed_password -> Text,
        is_validated -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
    }
}
