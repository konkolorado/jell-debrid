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
