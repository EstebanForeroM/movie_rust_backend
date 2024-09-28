-- Add migration script here

ALTER TABLE language ADD UNIQUE (language_name);
ALTER TABLE country ADD UNIQUE (country_name);
ALTER TABLE genre ADD UNIQUE (genre_name);

