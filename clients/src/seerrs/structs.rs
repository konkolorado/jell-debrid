use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct NoContent {}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub page: u64,
    pub pages: u64,
    pub results: u64,
    pub page_size: u64,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    id: u64,
    tmdb_id: u64,
    tvdb_id: Option<u64>,
    status: u8,
    //requests: Vec<HashMap<K, V>>,
    created_at: String,
    updated_at: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    avatar: String,
    created_at: String,
    display_name: String,
    email: String,
    id: u8,
    jellyfin_auth_token: String,
    jellyfin_device_id: String,
    jellyfin_user_id: String,
    jellyfin_username: String,
    movie_quota_days: Option<u8>,
    movie_quota_limit: Option<u8>,
    permissions: u8,
    plex_id: Option<u16>,
    plex_token: Option<String>,
    plex_username: Option<String>,
    recovery_link_expiration_date: Option<String>,
    request_count: u8,
    tv_quota_days: Option<u16>,
    tv_quota_limit: Option<u16>,
    updated_at: String,
    user_type: u8,
    username: Option<String>,
    warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum UserOrString {
    User(User),
    String(String),
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaRequest {
    pub created_at: String,
    pub id: u64,
    pub is4k: bool,
    pub media: MediaInfo,
    pub modified_by: Option<UserOrString>,
    pub profile_id: Option<u64>,
    pub requested_by: User,
    pub root_folder: Option<String>,
    pub server_id: Option<u64>,
    pub status: u8,
    pub r#type: String,
    pub updated_at: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Requests {
    pub page_info: PageInfo,
    pub results: Vec<MediaRequest>,
}
