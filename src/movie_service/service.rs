use super::domain::MovieConstructor;
use super::movie_database::{MovieDataDb, MovieDb};
use super::error::{self, Result};


pub async fn create_movie(database: MovieDb, movie_constructor: MovieConstructor) -> Result<()> {

    let original_language_id = database.get_language_id(movie_constructor.original_language)
        .await?.ok_or(error::MovieServiceError::InvalidLanguageName)?;

    let origin_country_id = database.get_country_id(movie_constructor.origin_country)
        .await?.ok_or(error::MovieServiceError::InvalidCountryName)?;

    let genre_id = database.get_genre_id(movie_constructor.genre)
        .await?.ok_or(error::MovieServiceError::InvalidGenreName)?;

    let classification_id = database.get_classification_id(movie_constructor.classification)
        .await?.ok_or(error::MovieServiceError::InvalidClassificationName)?;

    let movie_database_constructor = MovieDataDb {
        distribution_title: movie_constructor.distribution_title,
        original_title: movie_constructor.original_title,
        original_language_id,
        has_spanish_subtitles: movie_constructor.has_spanish_subtitles,
        production_year: movie_constructor.production_year,
        website_url: movie_constructor.website_url,
        image_url: movie_constructor.image_url,
        duration_hours: movie_constructor.duration_hours,
        summary: movie_constructor.summary,
        origin_country_id,
        genre_id,
        classification_id
    };

    database.insert_movie(&movie_database_constructor).await?;

    Ok(())
}
