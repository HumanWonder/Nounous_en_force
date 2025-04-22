-- This file should undo anything in `up.sql`
ALTER TABLE nurseries
ALTER COLUMN id DROP DEFAULT;

ALTER TABLE nursery_description
ALTER COLUMN id DROP DEFAULT;

ALTER TABLE nursery_responsibles
ALTER COLUMN id DROP DEFAULT;

ALTER TABLE replacement_needs
ALTER COLUMN id DROP DEFAULT;