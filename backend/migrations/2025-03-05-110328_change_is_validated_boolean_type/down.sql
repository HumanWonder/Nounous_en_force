-- This file should undo anything in `up.sql`
-- Annule la migration en rendant à nouveau la colonne nullable
ALTER TABLE users
    ALTER COLUMN is_validated DROP NOT NULL,
    ALTER COLUMN is_validated DROP DEFAULT;

ALTER TABLE users
    ALTER COLUMN is_profile_validated DROP NOT NULL,
    ALTER COLUMN is_profile_validated DROP DEFAULT;
