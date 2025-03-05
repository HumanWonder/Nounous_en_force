// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        hashed_password -> Text,
        is_validated -> Bool,
        created_at -> Nullable<Timestamp>,
        #[max_length = 50]
        role -> Varchar,
        is_profile_validated -> Bool,
    }
}
