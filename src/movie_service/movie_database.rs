use sqlx::PgPool;

use super::domain::{BasicMovie, Country, Genre, Language, Movie};
use super::error::Result;


pub struct MovieDb {
    pool: PgPool
}

impl MovieDb {
    pub fn new(pool: PgPool) -> MovieDb {
        MovieDb { pool }
    }

    pub async fn get_basic_movie_page(&self, page: i64, quantity: i64) -> Result<Vec<BasicMovie>> {

        let offset = page * quantity;

        let movies = sqlx::query_as!(BasicMovie, "SELECT 
movie_id, distribution_title, image_url FROM movie OFFSET $1 LIMIT $2", offset, quantity)
            .fetch_all(&self.pool).await?;        

        Ok(movies)
    }

    pub async fn get_movie_page(&self, page: i64, quantity: i64) -> Result<Vec<Movie>> {

        let offset = page * quantity;

        let movies = sqlx::query_as!(Movie, "SELECT 
m.movie_id, m.distribution_title, m.original_title, l.language_name AS original_language,
m.has_spanish_subtitles, m.production_year, m.website_url, m.image_url, m.duration_hours,
m.summary, c.classification_name AS classification FROM movie m
INNER JOIN language l ON l.language_id = m.original_language_id
INNER JOIN classification c ON c.classification_id = m.classification_id
OFFSET $1 LIMIT $2", offset, quantity)
            .fetch_all(&self.pool).await?;        

        Ok(movies)
    }

    pub async fn get_movie(&self, movie_id: i32) -> Result<Movie> {
        let country = sqlx::query_as!(Movie, "SELECT 
m.movie_id, m.distribution_title, m.original_title, l.language_name AS original_language,
m.has_spanish_subtitles, m.production_year, m.website_url, m.image_url, m.duration_hours,
m.summary, c.classification_name AS classification FROM movie m
INNER JOIN language l ON l.language_id = m.original_language_id
INNER JOIN classification c ON c.classification_id = m.classification_id
WHERE m.movie_id = $1", movie_id)
            .fetch_one(&self.pool).await?;

        Ok(country)
    }

    pub async fn get_languages(&self) -> Result<Vec<Language>> {
        let languages = sqlx::query_as!(Language, "SELECT * FROM language")
            .fetch_all(&self.pool).await?;

        Ok(languages)
    }

    pub async fn get_language(&self, language_id: i32) -> Result<Language> {
        let country = sqlx::query_as!(Language, "SELECT * FROM language WHERE language_id = $1", language_id)
            .fetch_one(&self.pool).await?;

        Ok(country)
    }

    pub async fn get_countries(&self) -> Result<Vec<Country>> {
        let countries = sqlx::query_as!(Country, "SELECT * FROM country")
            .fetch_all(&self.pool).await?;

        Ok(countries)
    }

    pub async fn get_country(&self, country_id: i32) -> Result<Country> {
        let country = sqlx::query_as!(Country, "SELECT * FROM country WHERE country_id = $1", country_id)
            .fetch_one(&self.pool).await?;

        Ok(country)
    }

    pub async fn get_genres(&self) -> Result<Vec<Genre>> {
        let genres = sqlx::query_as!(Genre, "SELECT * FROM genre")
            .fetch_all(&self.pool).await?;

        Ok(genres)
    }

    pub async fn get_genre(&self, genre_id: i32) -> Result<Genre> {
        let genres = sqlx::query_as!(Genre, "SELECT * FROM genre WHERE genre_id = $1", genre_id)
            .fetch_one(&self.pool).await?;

        Ok(genres)
    }
}
