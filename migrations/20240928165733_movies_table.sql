-- Add migration script here

CREATE TABLE language (
    languageId SERIAL PRIMARY KEY,
    languageName VARCHAR(20) NOT NULL
);

CREATE TABLE classification (
    classificationId SERIAL PRIMARY KEY,
    classificationName VARCHAR(20) NOT NULL
);

CREATE TABLE movie (
    movieId SERIAL PRIMARY KEY,
    distributionTitle VARCHAR(40) NOT NULL,
    originalTitle VARCHAR(40)  NOT NULL,
    originalLanguageId INTEGER  NOT NULL,
    hasSpanishSubtitles BOOLEAN  NOT NULL,
    productionYear INTEGER  NOT NULL,
    websiteURL VARCHAR(100) NOT NULL,
    imageURL VARCHAR(100) NOT NULL,
    durationHours INTEGER NOT NULL,
    summary TEXT,
    classificationId INTEGER NOT NULL,
    FOREIGN KEY (originalLanguageId) REFERENCES language(languageId),
    FOREIGN KEY (classificationId) REFERENCES classification(classificationId)
);

CREATE TABLE country (
    countryId SERIAL PRIMARY KEY,
    countryName VARCHAR(30) NOT NULL
);

CREATE TABLE movie_country (
    movieId INTEGER NOT NULL,
    countryId INTEGER NOT NULL,
    PRIMARY KEY (movieId, countryId),
    FOREIGN KEY (movieId) REFERENCES movie(movieId),
    FOREIGN KEY (countryId) REFERENCES country(countryId)
);

CREATE TABLE gender (
    genderId SERIAL PRIMARY KEY,
    genderName VARCHAR(30) NOT NULL
);

CREATE TABLE movie_gender (
    genderId INTEGER NOT NULL,
    movieId INTEGER NOT NULL,
    PRIMARY KEY (genderId, movieId),
    FOREIGN KEY (genderId) REFERENCES gender(genderId),
    FOREIGN KEY (movieId) REFERENCES movie(movieId)
);
