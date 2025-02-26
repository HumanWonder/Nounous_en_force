-- This file should undo anything in `up.sql`
-- Suppression de la colonne ajoutée
ALTER TABLE users DROP COLUMN IF EXISTS is_profile_validated;