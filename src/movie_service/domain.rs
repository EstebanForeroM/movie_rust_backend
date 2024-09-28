use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Genre {
    pub genre_id: i32,
    pub genre_name: String
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GenreConstructor {
    pub genre_name: String
}

impl GenreConstructor {
    pub fn to_genre(self) -> Genre {
        Genre {genre_id: 0, genre_name: self.genre_name}
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Country {
    pub country_id: i32,
    pub country_name: String
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CountryConstructor {
    pub country_name: String
}

impl CountryConstructor {
    pub fn to_country(self) -> Country {
        Country { country_id: 0, country_name: self.country_name }
    } 
}

#[derive(Debug, Serialize, Clone)]
pub struct Language {
    pub language_id: i32,
    pub language_name: String
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LanguageConstructor {
    pub language_name: String
}

impl LanguageConstructor {
    pub fn to_language(self) -> Language {
        Language { language_id: 0, language_name: self.language_name }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Movie {
    pub movie_id: i32,
    pub distribution_title: String,
    pub original_title: String,
    pub original_language: String,
    pub has_spanish_subtitles: bool,
    pub production_year: i32,
    pub website_url: String,
    pub image_url: String,
    pub duration_hours: i32,
    pub summary: Option<String>,
    pub classification: String
}

#[derive(Debug, Serialize, Clone)]
pub struct BasicMovie {
    pub movie_id: i32,
    pub distribution_title: String,
    pub image_url: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct MovieConstructor {
    pub distribution_title: String,
    pub original_title: String,
    pub original_language: String,
    pub has_spanish_subtitles: bool,
    pub production_year: i32,
    pub website_url: String,
    pub image_url: String,
    pub duration_hours: i32,
    pub summary: Option<String>,
    pub classification: String,
    pub origin_country: String,
    pub genre: String,
}

impl MovieConstructor {
    pub fn to_movie(self) -> Movie {
        Movie {
            movie_id: 0,
            distribution_title: self.distribution_title,
            original_title: self.original_title,
            original_language: self.original_language,
            has_spanish_subtitles: self.has_spanish_subtitles,
            production_year: self.production_year,
            website_url: self.website_url,
            image_url: self.image_url,
            duration_hours: self.duration_hours,
            summary: self.summary,
            classification: self.classification
        }
    }
}
