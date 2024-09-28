use sqlx::{pool, PgPool};

use super::domain::{BasicMovie, Country, Genre, Language, Movie};
use super::error::Result;

pub struct MovieDb {
    pool: PgPool
}

pub struct MovieDataDb {
    pub distribution_title: String,
    pub original_title: String,
    pub original_language_id: i32,
    pub has_spanish_subtitles: bool,
    pub production_year: i32,
    pub website_url: String,
    pub image_url: String,
    pub duration_hours: i32,
    pub summary: Option<String>,
    pub origin_country_id: i32,
    pub genre_id: i32,
    pub classification_id: i32,
}

impl MovieDb {
    pub fn new(pool: PgPool) -> MovieDb {
        MovieDb { pool }
    }

    pub async fn insert_movie(&self, movie: &MovieDataDb) -> Result<()> {

        let mut tx = self.pool.begin().await?;

        let movie_id = sqlx::query_scalar!(
            "
        INSERT INTO movie (
            distribution_title, original_title, original_language_id, 
            has_spanish_subtitles, production_year, website_url, image_url, 
            duration_hours, summary, classification_id
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
        ) RETURNING movie_id
        ",
            movie.distribution_title,
            movie.original_title,
            movie.original_language_id,
            movie.has_spanish_subtitles,
            movie.production_year,
            movie.website_url,
            movie.image_url,
            movie.duration_hours,
            movie.summary,
            movie.classification_id
        )
            .fetch_one(&mut tx)
        .await?;

        sqlx::query!(
            "INSERT INTO movie_country(movie_id, country_id) VALUES ($1, $2)",
            movie_id, movie.origin_country_id
        ).execute(&mut tx).await?;

        sqlx::query!(
            "INSERT INTO movie_genre(movie_id, genre_id) VALUES ($1, $2)",
            movie_id, movie.genre_id
        ).execute(&mut tx).await?;

        tx.commit().await?;

        Ok(())
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
        let language = sqlx::query_as!(Language, "SELECT * FROM language WHERE language_id = $1", language_id)
            .fetch_one(&self.pool).await?;

        Ok(language)
    }

    pub async fn get_language_id(&self, language_name: String) -> Result<i32> {
        let language_id = sqlx::query_scalar!("SELECT language_id FROM language WHERE language_name = $1", language_name)
            .fetch_optional(&self.pool).await?;

        Ok(language_id)
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

    pub async fn get_country_id(&self, country_name: String) -> Result<i32> {
        let country_id = sqlx::query_scalar!("SELECT country_id FROM country WHERE country_name = $1", country_name)
            .fetch_optional(&self.pool).await?;

        Ok(country_id)
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

    pub async fn get_genre_id(&self, genre_name: String) -> Result<i32> {
        let genre_id = sqlx::query_scalar!("SELECT country_id FROM country WHERE country_name = $1", genre_name)
            .fetch_optional(&self.pool).await?;

        Ok(genre_id)
    }
}
