-- This file should undo anything in `up.sql`
ALTER TABLE temps ADD COLUMN email TEXT NOT NULL;
