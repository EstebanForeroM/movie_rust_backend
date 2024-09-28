-- Add migration script here

CREATE TABLE language (
    language_id SERIAL PRIMARY KEY,
    language_name VARCHAR(20) NOT NULL
);

CREATE TABLE classification (
    classification_id SERIAL PRIMARY KEY,
    classification_name VARCHAR(20) NOT NULL
);

CREATE TABLE movie (
    movie_id SERIAL PRIMARY KEY,
    distribution_title VARCHAR(40) NOT NULL,
    original_title VARCHAR(40) NOT NULL,
    original_language_id INTEGER NOT NULL,
    has_spanish_subtitles BOOLEAN NOT NULL,
    production_year INTEGER NOT NULL,
    website_url VARCHAR(100) NOT NULL,
    image_url VARCHAR(100) NOT NULL,
    duration_hours INTEGER NOT NULL,
    summary TEXT,
    classification_id INTEGER NOT NULL,
    FOREIGN KEY (original_language_id) REFERENCES language(language_id),
    FOREIGN KEY (classification_id) REFERENCES classification(classification_id)
);

CREATE TABLE country (
    country_id SERIAL PRIMARY KEY,
    country_name VARCHAR(30) NOT NULL
);

CREATE TABLE movie_country (
    movie_id INTEGER NOT NULL,
    country_id INTEGER NOT NULL,
    PRIMARY KEY (movie_id, country_id),
    FOREIGN KEY (movie_id) REFERENCES movie(movie_id),
    FOREIGN KEY (country_id) REFERENCES country(country_id)
);

CREATE TABLE genre (
    genre_id SERIAL PRIMARY KEY,
    genre_name VARCHAR(30) NOT NULL
);

CREATE TABLE movie_genre (
    genre_id INTEGER NOT NULL,
    movie_id INTEGER NOT NULL,
    PRIMARY KEY (genre_id, movie_id),
    FOREIGN KEY (genre_id) REFERENCES genre(genre_id),
    FOREIGN KEY (movie_id) REFERENCES movie(movie_id)
);

