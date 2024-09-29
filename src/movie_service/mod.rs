use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Extension, Json, Router};
use domain::{ClassificationConstructor, CountryConstructor, GenreConstructor, LanguageConstructor, MovieConstructor};
use movie_database::MovieDb;
use sqlx::PgPool;
use tracing::error;

use crate::auth_middleware::ClientInfo;

mod domain;
pub mod error;
mod movie_database;
mod service;

#[derive(Clone, Debug)]
struct MovieServiceState {
    db_pool: PgPool
}

pub fn get_router(db_pool: PgPool) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/genre", get(get_genres))
        .route("/genre/:genreId", get(get_genre))
        .route("/country", get(get_countries))
        .route("/country/:countryId", get(get_country))
        .route("/classification", get(get_classifications))
        .route("/classification/:classificationId", get(get_classification))
        .route("/language", get(get_languages))
        .route("/language/:languageId", get(get_language))
        .route("/movie/page/:pageIndex/:quantity", get(get_movies))
        .route("/movie/:movieId", get(get_movie))
        .route("/basic_data_movie/page/:pageIndex/:quantity", get(get_movie_basic_data))
        // POSTS
        .route("/language", post(create_language))
        .route("/classification", post(create_classification))
        .route("/country", post(create_country))
        .route("/genre", post(create_genre))
        .route("/movie", post(create_movie))
        .with_state(MovieServiceState {
            db_pool,
        })
}

async fn health_check(Extension(client_info): Extension<ClientInfo>) -> String {
    format!("movie service alive, and client name is: {}", client_info.client_name)
}

async fn create_language(State(state): State<MovieServiceState>, Json(language_constructor): Json<LanguageConstructor>) -> Result<impl IntoResponse, StatusCode> {
    let db = MovieDb::new(state.db_pool);

    db.create_language_db(language_constructor.language_name).await.map_err(|err| {
        error!("Error creating a lenguage: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}

async fn create_country(State(state): State<MovieServiceState>, Json(country_constructor): Json<CountryConstructor>) -> Result<impl IntoResponse, StatusCode> {
    let db = MovieDb::new(state.db_pool);

    db.create_country_db(country_constructor.country_name).await.map_err(|err| {
        error!("Error creating country: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}

async fn create_genre(State(state): State<MovieServiceState>, Json(genre_constructor): Json<GenreConstructor>) -> Result<impl IntoResponse, StatusCode> {
    let db = MovieDb::new(state.db_pool);

    db.create_genre_db(genre_constructor.genre_name).await.map_err(|err| {
        error!("Error creating gengre: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}

async fn create_movie(State(state): State<MovieServiceState>, Json(movie_constructor): Json<MovieConstructor>) -> Result<impl IntoResponse, StatusCode> {
    let movie_database = MovieDb::new(state.db_pool);

    service::create_movie(movie_database, movie_constructor).await.map_err(|err| {
        error!("Error creating movie: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}

async fn get_movie_basic_data(State(state): State<MovieServiceState>, Path((page, quantity)): Path<(i64, i64)>) -> Result<impl IntoResponse, StatusCode> {
    let movie_database = MovieDb::new(state.db_pool);

    let movies = movie_database.get_basic_movie_page(page, quantity)
        .await
        .map_err(|err| {
            error!("Error getting genres in the movie database: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let movies_json = serde_json::to_string(&movies).map_err(|err| {
        error!("Error mapping genres to string in serde_json: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, movies_json))
} 

// get classification 
async fn get_classifications(State(state): State<MovieServiceState>) -> Result<impl IntoResponse, StatusCode> {
    let db = MovieDb::new(state.db_pool);

    let classifications = db.get_classifications_db().await.map_err(|err| {
        error!("Error getting classifications from db: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let classifications = serde_json::to_string(&classifications).map_err(|err| {
        error!("Parsing classifications: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, classifications))
}

async fn get_classification(State(state): State<MovieServiceState>, Path(id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    let db = MovieDb::new(state.db_pool);

    let classifications = db.get_classification_db(id).await.map_err(|err| {
        error!("Error getting classifications from db: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let classifications = serde_json::to_string(&classifications).map_err(|err| {
        error!("Parsing classifications: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, classifications))
}

async fn create_classification(State(state): State<MovieServiceState>,
    Json(classification_constructor): Json<ClassificationConstructor>) -> Result<impl IntoResponse, StatusCode> {

    let db = MovieDb::new(state.db_pool);

    db.create_classification_db(classification_constructor.classification_name).await.map_err(|err| {
        error!("Error creating classification: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}

async fn get_movies(State(state): State<MovieServiceState>, Path((page, quantity)): Path<(i64, i64)>) -> Result<impl IntoResponse, StatusCode> {
    let movie_database = MovieDb::new(state.db_pool);

    let movies = movie_database.get_movie_page(page, quantity)
        .await
        .map_err(|err| {
            error!("Error getting genres in the movie database: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let movies_json = serde_json::to_string(&movies).map_err(|err| {
        error!("Error mapping genres to string in serde_json: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, movies_json))
} 

async fn get_movie(State(state): State<MovieServiceState>, Path(movie_id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    let movie_database = MovieDb::new(state.db_pool);

    let movie = movie_database.get_movie(movie_id)
        .await
        .map_err(|err| {
            error!("Error getting genres in the movie database: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let movie_json = serde_json::to_string(&movie).map_err(|err| {
        error!("Error mapping genres to string in serde_json: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, movie_json))
} 

async fn get_languages(State(state): State<MovieServiceState>) -> Result<impl IntoResponse, StatusCode> {
    let movie_database = MovieDb::new(state.db_pool);

    let languages = movie_database.get_languages()
        .await
        .map_err(|err| {
            error!("Error getting genres in the movie database: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let languages_json = serde_json::to_string(&languages).map_err(|err| {
        error!("Error mapping genres to string in serde_json: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, languages_json))
} 

async fn get_language(State(state): State<MovieServiceState>, Path(country_id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    let movie_database = MovieDb::new(state.db_pool);

    let language = movie_database.get_country(country_id)
        .await
        .map_err(|err| {
            error!("Error getting genres in the movie database: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let language_json = serde_json::to_string(&language).map_err(|err| {
        error!("Error mapping genres to string in serde_json: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, language_json))
} 

async fn get_countries(State(state): State<MovieServiceState>) -> Result<impl IntoResponse, StatusCode> {
    let movie_database = MovieDb::new(state.db_pool);

    let countries = movie_database.get_countries()
        .await
        .map_err(|err| {
            error!("Error getting genres in the movie database: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let countries_json = serde_json::to_string(&countries).map_err(|err| {
        error!("Error mapping genres to string in serde_json: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, countries_json))
} 

async fn get_country(State(state): State<MovieServiceState>, Path(country_id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    let movie_database = MovieDb::new(state.db_pool);

    let country = movie_database.get_country(country_id)
        .await
        .map_err(|err| {
            error!("Error getting genres in the movie database: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let country_json = serde_json::to_string(&country).map_err(|err| {
        error!("Error mapping genres to string in serde_json: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, country_json))
} 

async fn get_genres(State(state): State<MovieServiceState>) -> Result<impl IntoResponse, StatusCode> {
    let movie_database = MovieDb::new(state.db_pool);

    let genres = movie_database.get_genres()
        .await
        .map_err(|err| {
            error!("Error getting genres in the movie database: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let genres_json = serde_json::to_string(&genres).map_err(|err| {
        error!("Error mapping genres to string in serde_json: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, genres_json))
}

async fn get_genre(State(state): State<MovieServiceState>, Path(genre_id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    let movie_database = MovieDb::new(state.db_pool);

    let genre = movie_database.get_genre(genre_id)
        .await
        .map_err(|err| {
            error!("Error getting genres in the movie database: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let genre_json = serde_json::to_string(&genre).map_err(|err| {
        error!("Error mapping genres to string in serde_json: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::OK, genre_json))
}
