-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL,
    hashed_password TEXT NOT NULL,
    is_validated BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP,
    role VARCHAR(50) NOT NULL,
    is_profile_validated BOOLEAN NOT NULL DEFAULT FALSE,
    rejection_reason TEXT
);

-- Table pour les intervenants (temps)
CREATE TABLE temps (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    address TEXT NOT NULL,
    phone TEXT NOT NULL,
    email TEXT NOT NULL,
    birth_date DATE,
    has_driver_license BOOLEAN NOT NULL,
    transport_mode TEXT NOT NULL
);

CREATE TABLE temp_diplomas (
    id UUID PRIMARY KEY,
    temp_id UUID REFERENCES temps(id) ON DELETE CASCADE,
    main_diploma TEXT NOT NULL,
    other_certifications TEXT,
    graduation_year INTEGER,
    school TEXT
);

CREATE TABLE temp_experiences (
    id UUID PRIMARY KEY,
    temp_id UUID REFERENCES temps(id) ON DELETE CASCADE,
    total_experience TEXT,
    previous_positions TEXT,
    structure_types TEXT,
    tasks TEXT
);

CREATE TABLE temp_skills (
    id UUID PRIMARY KEY,
    temp_id UUID REFERENCES temps(id) ON DELETE CASCADE,
    languages TEXT,
    pedagogies TEXT,
    special_skills TEXT,
    special_needs_handling TEXT
);

CREATE TABLE temp_availabilities (
    id UUID PRIMARY KEY,
    temp_id UUID REFERENCES temps(id) ON DELETE CASCADE,
    availability_periods TEXT,
    time_slots TEXT,
    geographic_zones TEXT,
    max_travel_time TEXT
);

CREATE TABLE temp_conditions (
    id UUID PRIMARY KEY,
    temp_id UUID REFERENCES temps(id) ON DELETE CASCADE,
    hourly_rate TEXT,
    contract_types TEXT,
    auto_entrepreneur BOOLEAN
);

CREATE TABLE temp_documents (
    id UUID PRIMARY KEY,
    temp_id UUID REFERENCES temps(id) ON DELETE CASCADE,
    motivation_letter TEXT,
    professional_references TEXT,
    required_documents TEXT,
    criminal_record TEXT,
    diplomas TEXT
);

-- Table pour les crèches
CREATE TABLE nurseries (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    address TEXT NOT NULL,
    phone TEXT,
    email TEXT,
    website TEXT,
    structure_type TEXT
);

CREATE TABLE nursery_responsibles (
    id UUID PRIMARY KEY,
    nursery_id UUID REFERENCES nurseries(id) ON DELETE CASCADE,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    role TEXT,
    direct_phone TEXT,
    direct_email TEXT
);

CREATE TABLE replacement_needs (
    id UUID PRIMARY KEY,
    nursery_id UUID REFERENCES nurseries(id) ON DELETE CASCADE,
    searched_position TEXT,
    replacement_reason TEXT,
    estimated_duration TEXT,
    available_periods TEXT,
    hours_per_week TEXT,
    main_tasks TEXT,
    required_skills TEXT,
    suggested_rate TEXT
);

CREATE TABLE nursery_description (
    id UUID PRIMARY KEY,
    nursery_id UUID REFERENCES nurseries(id) ON DELETE CASCADE,
    pedagogy TEXT,
    specificities TEXT,
    philosophy TEXT
);