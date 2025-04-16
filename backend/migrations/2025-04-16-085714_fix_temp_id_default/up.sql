-- Your SQL goes here
-- Modifie la colonne id pour avoir le bon DEFAULT
ALTER TABLE temps 
ALTER COLUMN id SET DEFAULT gen_random_uuid();

-- Ajout des defaults pour les IDs UUID
ALTER TABLE temp_availabilities
ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE temp_conditions
ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE temp_diplomas
ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE temp_experiences
ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE temp_documents
ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE temp_skills
ALTER COLUMN id SET DEFAULT gen_random_uuid();