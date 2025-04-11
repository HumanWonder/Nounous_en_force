-- Your SQL goes here
-- Active l'extension si elle ne l'est pas encore
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Modifie la colonne id pour avoir le bon DEFAULT
ALTER TABLE users 
ALTER COLUMN id SET DEFAULT gen_random_uuid();