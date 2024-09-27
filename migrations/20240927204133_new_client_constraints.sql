-- Add migration script here

ALTER TABLE client ALTER COLUMN client_name SET NOT NULL;
ALTER TABLE client ALTER COLUMN encrypted_password SET NOT NULL;
