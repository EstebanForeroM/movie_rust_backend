use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Genre {
    pub genre_id: i32,
    pub genre_name: String
}

#[derive(Debug, Serialize, Clone)]
pub struct Country {
    pub country_id: i32,
    pub country_name: String
}

#[derive(Debug, Serialize, Clone)]
pub struct Language {
    pub language_id: i32,
    pub language_name: String
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

#[derive(Debug, Serialize, Clone)]
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
    pub classification: String
}
