// @generated automatically by Diesel CLI.

diesel::table! {
    nurseries (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        name -> Text,
        address -> Text,
        phone -> Nullable<Text>,
        email -> Nullable<Text>,
        website -> Nullable<Text>,
        structure_type -> Nullable<Text>,
    }
}

diesel::table! {
    nursery_description (id) {
        id -> Uuid,
        nursery_id -> Nullable<Uuid>,
        pedagogy -> Nullable<Text>,
        specificities -> Nullable<Text>,
        philosophy -> Nullable<Text>,
    }
}

diesel::table! {
    nursery_responsibles (id) {
        id -> Uuid,
        nursery_id -> Nullable<Uuid>,
        first_name -> Text,
        last_name -> Text,
        role -> Nullable<Text>,
        direct_phone -> Nullable<Text>,
        direct_email -> Nullable<Text>,
    }
}

diesel::table! {
    replacement_needs (id) {
        id -> Uuid,
        nursery_id -> Nullable<Uuid>,
        searched_position -> Nullable<Text>,
        replacement_reason -> Nullable<Text>,
        estimated_duration -> Nullable<Text>,
        available_periods -> Nullable<Text>,
        hours_per_week -> Nullable<Text>,
        main_tasks -> Nullable<Text>,
        required_skills -> Nullable<Text>,
        suggested_rate -> Nullable<Text>,
    }
}

diesel::table! {
    temp_availabilities (id) {
        id -> Uuid,
        temp_id -> Nullable<Uuid>,
        availability_periods -> Nullable<Text>,
        time_slots -> Nullable<Text>,
        geographic_zones -> Nullable<Text>,
        max_travel_time -> Nullable<Text>,
    }
}

diesel::table! {
    temp_conditions (id) {
        id -> Uuid,
        temp_id -> Nullable<Uuid>,
        hourly_rate -> Nullable<Text>,
        contract_types -> Nullable<Text>,
        auto_entrepreneur -> Nullable<Bool>,
    }
}

diesel::table! {
    temp_diplomas (id) {
        id -> Uuid,
        temp_id -> Nullable<Uuid>,
        main_diploma -> Text,
        other_certifications -> Nullable<Text>,
        graduation_year -> Nullable<Int4>,
        school -> Nullable<Text>,
    }
}

diesel::table! {
    temp_documents (id) {
        id -> Uuid,
        temp_id -> Nullable<Uuid>,
        motivation_letter -> Nullable<Text>,
        professional_references -> Nullable<Text>,
        required_documents -> Nullable<Text>,
        criminal_record -> Nullable<Text>,
        diplomas -> Nullable<Text>,
    }
}

diesel::table! {
    temp_experiences (id) {
        id -> Uuid,
        temp_id -> Nullable<Uuid>,
        total_experience -> Nullable<Text>,
        previous_positions -> Nullable<Text>,
        structure_types -> Nullable<Text>,
        tasks -> Nullable<Text>,
    }
}

diesel::table! {
    temp_skills (id) {
        id -> Uuid,
        temp_id -> Nullable<Uuid>,
        languages -> Nullable<Text>,
        pedagogies -> Nullable<Text>,
        special_skills -> Nullable<Text>,
        special_needs_handling -> Nullable<Text>,
    }
}

diesel::table! {
    temps (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        first_name -> Text,
        last_name -> Text,
        address -> Text,
        phone -> Text,
        email -> Text,
        birth_date -> Nullable<Date>,
        has_driver_license -> Bool,
        transport_mode -> Text,
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

diesel::joinable!(nurseries -> users (user_id));
diesel::joinable!(nursery_description -> nurseries (nursery_id));
diesel::joinable!(nursery_responsibles -> nurseries (nursery_id));
diesel::joinable!(replacement_needs -> nurseries (nursery_id));
diesel::joinable!(temp_availabilities -> temps (temp_id));
diesel::joinable!(temp_conditions -> temps (temp_id));
diesel::joinable!(temp_diplomas -> temps (temp_id));
diesel::joinable!(temp_documents -> temps (temp_id));
diesel::joinable!(temp_experiences -> temps (temp_id));
diesel::joinable!(temp_skills -> temps (temp_id));
diesel::joinable!(temps -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    nurseries,
    nursery_description,
    nursery_responsibles,
    replacement_needs,
    temp_availabilities,
    temp_conditions,
    temp_diplomas,
    temp_documents,
    temp_experiences,
    temp_skills,
    temps,
    users,
);
