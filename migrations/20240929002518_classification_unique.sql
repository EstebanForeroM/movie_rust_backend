-- Add migration script here

ALTER TABLE classification ADD UNIQUE (classification_name);
