-- This file should undo anything in `up.sql`
ALTER TABLE temps 
ALTER COLUMN id DROP DEFAULT;

-- Ajout des defaults pour les IDs UUID
ALTER TABLE temp_availabilities
ALTER COLUMN id DROP DEFAULT;

ALTER TABLE temp_conditions
ALTER COLUMN id DROP DEFAULT;

ALTER TABLE temp_diplomas
ALTER COLUMN id DROP DEFAULT;

ALTER TABLE temp_experiences
ALTER COLUMN id DROP DEFAULT;

ALTER TABLE temp_documents
ALTER COLUMN id DROP DEFAULT;

ALTER TABLE temp_skills
ALTER COLUMN id DROP DEFAULT;