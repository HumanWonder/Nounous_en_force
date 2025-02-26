// @generated automatically by Diesel CLI.

diesel::table! {
    nurseries (id) {
        id -> Int4,
        referent_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        address -> Text,
        organization_type -> Nullable<Text>,
        #[max_length = 20]
        tel_number -> Nullable<Varchar>,
        mail_address -> Nullable<Text>,
        website -> Nullable<Text>,
    }
}

diesel::table! {
    owners (id) {
        id -> Int4,
        client_id -> Int4,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        job_position -> Nullable<Text>,
        #[max_length = 20]
        tel_number -> Nullable<Varchar>,
        address -> Nullable<Text>,
    }
}

diesel::table! {
    temps (id) {
        id -> Int4,
        client_id -> Int4,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 20]
        tel_number -> Nullable<Varchar>,
        address -> Nullable<Text>,
        disponibilities -> Jsonb,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        hashed_password -> Text,
        is_validated -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        #[max_length = 50]
        role -> Varchar,
        is_profile_validated -> Nullable<Bool>,
    }
}

diesel::table! {
    work_schedule (id) {
        id -> Int4,
        nursery_id -> Int4,
        date -> Timestamp,
        address -> Text,
    }
}

diesel::joinable!(owners -> users (client_id));
diesel::joinable!(temps -> users (client_id));
diesel::joinable!(work_schedule -> nurseries (nursery_id));

diesel::allow_tables_to_appear_in_same_query!(
    nurseries,
    owners,
    temps,
    users,
    work_schedule,
);
