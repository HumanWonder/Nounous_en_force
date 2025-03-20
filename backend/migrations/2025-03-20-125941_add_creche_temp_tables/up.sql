-- Your SQL goes here
-- Table des crèches
CREATE TABLE creches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    address TEXT NOT NULL,
    phone TEXT NOT NULL,
    email TEXT NOT NULL,
    website TEXT,
    structure_type TEXT NOT NULL,
    pedagogy TEXT,
    special_features TEXT,
    environment TEXT
);

-- Table des responsables de crèches
CREATE TABLE creche_responsables (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    creche_id UUID NOT NULL REFERENCES creches(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    role TEXT NOT NULL,
    phone TEXT NOT NULL,
    email TEXT NOT NULL
);

-- Table des besoins en remplacement
CREATE TABLE replacement_needs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    creche_id UUID NOT NULL REFERENCES creches(id) ON DELETE CASCADE,
    position TEXT NOT NULL,
    reason TEXT NOT NULL,
    estimated_duration TEXT NOT NULL,
    availability TEXT NOT NULL,
    weekly_hours INTEGER NOT NULL,
    tasks TEXT NOT NULL,
    required_skills TEXT NOT NULL,
    salary_range TEXT
);

-- Table des intervenants / remplaçants
CREATE TABLE temps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    full_name TEXT NOT NULL,
    address TEXT NOT NULL,
    phone TEXT NOT NULL,
    email TEXT NOT NULL,
    birth_date DATE,
    driver_license BOOLEAN NOT NULL,
    transport TEXT NOT NULL,
    motivation TEXT,
    judicial_record TEXT NOT NULL
);

-- Table des diplômes des intervenants
CREATE TABLE temp_diplomas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    temp_id UUID NOT NULL REFERENCES temps(id) ON DELETE CASCADE,
    diploma_name TEXT NOT NULL,
    other_certifications TEXT,
    year_obtained INTEGER NOT NULL,
    institution TEXT NOT NULL
);

-- Table des expériences professionnelles des intervenants
CREATE TABLE temp_experiences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    temp_id UUID NOT NULL REFERENCES temps(id) ON DELETE CASCADE,
    total_experience TEXT NOT NULL,
    previous_jobs TEXT NOT NULL,
    structure_types TEXT NOT NULL,
    tasks TEXT NOT NULL
);

-- Table des disponibilités des intervenants
CREATE TABLE temp_availabilities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    temp_id UUID NOT NULL REFERENCES temps(id) ON DELETE CASCADE,
    available_periods TEXT NOT NULL,
    work_hours TEXT NOT NULL,
    preferred_locations TEXT NOT NULL,
    max_travel_time TEXT NOT NULL
);

-- Table des conditions de travail des intervenants
CREATE TABLE temp_conditions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    temp_id UUID NOT NULL REFERENCES temps(id) ON DELETE CASCADE,
    hourly_rate TEXT NOT NULL,
    contract_types TEXT NOT NULL,
    self_employment BOOLEAN NOT NULL
);
