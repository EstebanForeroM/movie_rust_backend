use sqlx::{pool, PgPool};
use tracing::error;

use super::domain::{BasicMovie, Classification, Country, Genre, Language, Movie, MovieConstructor};
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
m.summary, c.classification_name AS classification, co.country_name AS origin_country,
g.genre_name AS genre FROM movie m
INNER JOIN language l ON l.language_id = m.original_language_id
INNER JOIN classification c ON c.classification_id = m.classification_id
INNER JOIN movie_genre mg ON mg.movie_id = m.movie_id
INNER JOIN genre g ON g.genre_id = mg.genre_id
INNER JOIN movie_country mc ON mc.movie_id = m.movie_id
INNER JOIN country co ON co.country_id = mc.country_id
OFFSET $1 LIMIT $2", offset, quantity)
            .fetch_all(&self.pool).await?;        

        Ok(movies)
    }

    pub async fn get_movie(&self, movie_id: i32) -> Result<Movie> {
        let country = sqlx::query_as!(Movie, "SELECT 
m.movie_id, m.distribution_title, m.original_title, l.language_name AS original_language,
m.has_spanish_subtitles, m.production_year, m.website_url, m.image_url, m.duration_hours,
m.summary, c.classification_name AS classification, co.country_name AS origin_country,
g.genre_name AS genre FROM movie m
INNER JOIN language l ON l.language_id = m.original_language_id
INNER JOIN classification c ON c.classification_id = m.classification_id
INNER JOIN movie_genre mg ON mg.movie_id = m.movie_id
INNER JOIN genre g ON g.genre_id = mg.genre_id
INNER JOIN movie_country mc ON mc.movie_id = m.movie_id
INNER JOIN country co ON co.country_id = mc.country_id
WHERE m.movie_id = $1", movie_id)
            .fetch_one(&self.pool).await?;

        Ok(country)
    }

    // languages
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

    pub async fn get_language_id(&self, language_name: String) -> Result<Option<i32>> {
        let language_id = sqlx::query_scalar!("SELECT language_id FROM language WHERE language_name = $1", language_name)
            .fetch_optional(&self.pool).await?;

        Ok(language_id)
    }

    pub async fn create_language_db(&self, language_name: String) -> Result<()> {
        sqlx::query!("INSERT INTO language(language_name) VALUES($1)", language_name).execute(&self.pool).await?;
        Ok(())
    }

    // countries
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

    pub async fn create_country_db(&self, country_name: String) -> Result<()> {
        sqlx::query!("INSERT INTO country(country_name) VALUES($1)", country_name).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn get_country_id(&self, country_name: String) -> Result<Option<i32>> {
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
    
    pub async fn create_genre_db(&self, genre_name: String) -> Result<()> {
        sqlx::query!("INSERT INTO genre(genre_name) VALUES($1)", genre_name).execute(&self.pool).await?;
        Ok(())
    }

    // classifications
    pub async fn get_classifications_db(&self) -> Result<Vec<Classification>> {
        let classifications = sqlx::query_as!(Classification, "SELECT * FROM classification").fetch_all(&self.pool).await?;
        Ok(classifications)
    }

    pub async fn get_classification_db(&self, classification_id: i32) -> Result<Classification> {
        let classification = sqlx::query_as!(Classification, "SELECT * FROM classification WHERE classification_id = $1", classification_id)
        .fetch_one(&self.pool).await?;

        Ok(classification)
    }

    pub async fn get_classification_id(&self, classification_name: String) -> Result<Option<i32>> {
        let classification_id = sqlx::query_scalar!("SELECT classification_id FROM classification WHERE classification_name = $1", classification_name)
            .fetch_optional(&self.pool).await?;

        Ok(classification_id)
    }

    pub async fn create_classification_db(&self, classification_name: String) -> Result<()> {
        sqlx::query!("INSERT INTO classification(classification_name) VALUES($1)", classification_name)
        .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn get_genre_id(&self, genre_name: String) -> Result<Option<i32>> {
        let genre_id = sqlx::query_scalar!("SELECT genre_id FROM genre WHERE genre_name = $1", genre_name)
            .fetch_optional(&self.pool).await?;

        Ok(genre_id)
    }

    pub async fn delete_movie_db(&self, movie_id: i32) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!("DELETE FROM movie_country WHERE movie_id = $1", movie_id).execute(&mut tx).await?;
        sqlx::query!("DELETE FROM movie_genre WHERE movie_id = $1", movie_id).execute(&mut tx).await?;
        sqlx::query!("DELETE FROM movie WHERE movie_id = $1", movie_id).execute(&mut tx).await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn update_movie_db(&self, movie: Movie) -> Result<()> {
        let classification_id = self.get_classification_id(movie.classification).await?;
        let language_id = self.get_language_id(movie.original_language).await?;
        let genre_id = self.get_genre_id(movie.genre).await?;
        let country_id = self.get_country_id(movie.origin_country).await?;

        let mut tx = self.pool.begin().await?;

        sqlx::query!("UPDATE movie_genre SET genre_id = $2 WHERE movie_id = $1", movie.movie_id, genre_id)
            .execute(&mut tx).await?;

        sqlx::query!("UPDATE movie_country SET country_id = $2 WHERE movie_id = $1", movie.movie_id, country_id)
            .execute(&mut tx).await?;

        sqlx::query!("UPDATE movie SET distribution_title = $1, original_title = $2, 
        original_language_id = $3, has_spanish_subtitles = $4, production_year = $5, website_url = $6,
        image_url = $7, duration_hours = $8, summary = $9, classification_id = $10 WHERE movie_id = $11", 
        movie.distribution_title, movie.original_title, language_id, movie.has_spanish_subtitles, 
        movie.production_year, movie.website_url, movie.image_url, movie.duration_hours, movie.summary,
        classification_id, movie.movie_id).execute(&mut tx).await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn get_movie_search_db(&self, movie_name: String) -> Result<Vec<Movie>> {
        let movie_name = format!("%{}%", movie_name);
        let movies = sqlx::query_as!(Movie, "SELECT 
m.movie_id, m.distribution_title, m.original_title, l.language_name AS original_language,
m.has_spanish_subtitles, m.production_year, m.website_url, m.image_url, m.duration_hours,
m.summary, c.classification_name AS classification, co.country_name AS origin_country,
g.genre_name AS genre FROM movie m
INNER JOIN language l ON l.language_id = m.original_language_id
INNER JOIN classification c ON c.classification_id = m.classification_id
INNER JOIN movie_genre mg ON mg.movie_id = m.movie_id
INNER JOIN genre g ON g.genre_id = mg.genre_id
INNER JOIN movie_country mc ON mc.movie_id = m.movie_id
INNER JOIN country co ON co.country_id = mc.country_id
WHERE distribution_title ILIKE $1 OR original_title ILIKE $1", movie_name).fetch_all(&self.pool).await?;
        Ok(movies)
    }
}
