-- Add migration script here

CREATE TABLE client(
    client_id SERIAL PRIMARY KEY,
    client_name VARCHAR(30) UNIQUE,
    encrypted_password VARCHAR(100)
);

CREATE INDEX idx_client ON client(client_name);
