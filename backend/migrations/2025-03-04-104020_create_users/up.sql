-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email TEXT NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL,
    is_validated BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT now(),
    role VARCHAR(50) NOT NULL,
    is_profile_validated BOOLEAN DEFAULT FALSE
);
