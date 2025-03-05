-- Your SQL goes here
-- On modifie la colonne is_validated pour qu'elle soit de type BOOLEAN non nullable, avec une valeur par défaut
-- Modification de la colonne is_validated
ALTER TABLE users
    ALTER COLUMN is_validated SET DATA TYPE BOOLEAN USING is_validated::BOOLEAN,
    ALTER COLUMN is_validated SET DEFAULT FALSE,
    ALTER COLUMN is_validated SET NOT NULL;

-- Modification de la colonne is_profile_validated
ALTER TABLE users
    ALTER COLUMN is_profile_validated SET DATA TYPE BOOLEAN USING is_profile_validated::BOOLEAN,
    ALTER COLUMN is_profile_validated SET DEFAULT FALSE,
    ALTER COLUMN is_profile_validated SET NOT NULL;
