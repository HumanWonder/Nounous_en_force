-- Your SQL goes here
ALTER TABLE nurseries
ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE nursery_responsibles
ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE nursery_description
ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE replacement_needs
ALTER COLUMN id SET DEFAULT gen_random_uuid();