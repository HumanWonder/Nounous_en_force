-- Your SQL goes here
-- Ajout du rôle aux utilisateurs
ALTER TABLE users ADD COLUMN role VARCHAR(50) NOT NULL DEFAULT 'user';
UPDATE users SET role = 'user' WHERE role IS NULL;

-- Table des propriétaires (responsables de crèches)
CREATE TABLE IF NOT EXISTS owners (
    id SERIAL PRIMARY KEY,
    client_id INT NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    last_name VARCHAR(255) NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    job_position TEXT,
    tel_number VARCHAR(20),
    address TEXT
);

-- Table des crèches
CREATE TABLE IF NOT EXISTS nurseries (
    id SERIAL PRIMARY KEY,
    referent_id INT NOT NULL REFERENCES owners(client_id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    address TEXT NOT NULL,
    organization_type TEXT,
    tel_number VARCHAR(20),
    mail_address TEXT,
    website TEXT
);

-- Table des intérimaires
CREATE TABLE IF NOT EXISTS temps (
    id SERIAL PRIMARY KEY,
    client_id INT NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    last_name VARCHAR(255) NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    tel_number VARCHAR(20),
    address TEXT,
    disponibilities JSONB NOT NULL DEFAULT '{}'
);

-- Table des plannings de travail
CREATE TABLE IF NOT EXISTS work_schedule (
    id SERIAL PRIMARY KEY,
    nursery_id INT NOT NULL REFERENCES nurseries(id) ON DELETE CASCADE,
    date TIMESTAMP NOT NULL,
    address TEXT NOT NULL
);
