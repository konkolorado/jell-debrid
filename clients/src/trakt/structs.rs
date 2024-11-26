use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
#[serde(transparent)]
pub struct WatchList {
    pub watchlist: Vec<WatchListItem>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchListItem {
    pub r#type: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(transparent)]

pub struct SearchResult {
    pub searchresult: Vec<SearchResultItem>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum SearchResultItem {
    Movie(SearchResultMovie),
    Show(SearchResultShow),
    Episode(SearchResultEpisode),
    Person(SearchResultPerson),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultMovie {
    pub score: f32,
    pub movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub year: u32,
    pub ids: IDs,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultShow {
    pub score: f32,
    pub show: Show,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    pub title: String,
    pub year: u32,
    pub ids: IDs,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultEpisode {
    pub score: f32,
    pub episode: Episode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub title: String,
    pub ids: IDs,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultPerson {
    pub score: f32,
    pub person: Person,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub ids: IDs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IDs {
    pub imdb: Option<String>,
}
