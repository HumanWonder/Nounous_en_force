-- This file should undo anything in `up.sql`
--Lancé lorsqu'on utilise la commande 'diesel migration revert'
--Cela permet de supprimer les modifications et de revenir à la précédente
-- Suppression des tables créées
DROP TABLE IF EXISTS work_schedule;
DROP TABLE IF EXISTS temps;
DROP TABLE IF EXISTS nurseries;
DROP TABLE IF EXISTS owners;

-- Suppression de la colonne ajoutée
ALTER TABLE users DROP COLUMN IF EXISTS role;
