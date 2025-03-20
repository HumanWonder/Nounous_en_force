// @generated automatically by Diesel CLI.

diesel::table! {
    creche_responsables (id) {
        id -> Uuid,
        creche_id -> Uuid,
        name -> Text,
        role -> Text,
        phone -> Text,
        email -> Text,
    }
}

diesel::table! {
    creches (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Text,
        address -> Text,
        phone -> Text,
        email -> Text,
        website -> Nullable<Text>,
        structure_type -> Text,
        pedagogy -> Nullable<Text>,
        special_features -> Nullable<Text>,
        environment -> Nullable<Text>,
    }
}

diesel::table! {
    replacement_needs (id) {
        id -> Uuid,
        creche_id -> Uuid,
        position -> Text,
        reason -> Text,
        estimated_duration -> Text,
        availability -> Text,
        weekly_hours -> Int4,
        tasks -> Text,
        required_skills -> Text,
        salary_range -> Nullable<Text>,
    }
}

diesel::table! {
    temp_availabilities (id) {
        id -> Uuid,
        temp_id -> Uuid,
        available_periods -> Text,
        work_hours -> Text,
        preferred_locations -> Text,
        max_travel_time -> Text,
    }
}

diesel::table! {
    temp_conditions (id) {
        id -> Uuid,
        temp_id -> Uuid,
        hourly_rate -> Text,
        contract_types -> Text,
        self_employment -> Bool,
    }
}

diesel::table! {
    temp_diplomas (id) {
        id -> Uuid,
        temp_id -> Uuid,
        diploma_name -> Text,
        other_certifications -> Nullable<Text>,
        year_obtained -> Int4,
        institution -> Text,
    }
}

diesel::table! {
    temp_experiences (id) {
        id -> Uuid,
        temp_id -> Uuid,
        total_experience -> Text,
        previous_jobs -> Text,
        structure_types -> Text,
        tasks -> Text,
    }
}

diesel::table! {
    temps (id) {
        id -> Uuid,
        user_id -> Uuid,
        full_name -> Text,
        address -> Text,
        phone -> Text,
        email -> Text,
        birth_date -> Nullable<Date>,
        driver_license -> Bool,
        transport -> Text,
        motivation -> Nullable<Text>,
        judicial_record -> Text,
    }
}

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
        rejection_reason -> Nullable<Text>,
    }
}

diesel::joinable!(creche_responsables -> creches (creche_id));
diesel::joinable!(creches -> users (user_id));
diesel::joinable!(replacement_needs -> creches (creche_id));
diesel::joinable!(temp_availabilities -> temps (temp_id));
diesel::joinable!(temp_conditions -> temps (temp_id));
diesel::joinable!(temp_diplomas -> temps (temp_id));
diesel::joinable!(temp_experiences -> temps (temp_id));
diesel::joinable!(temps -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    creche_responsables,
    creches,
    replacement_needs,
    temp_availabilities,
    temp_conditions,
    temp_diplomas,
    temp_experiences,
    temps,
    users,
);
