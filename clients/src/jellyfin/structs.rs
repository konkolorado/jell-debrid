use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct NoContent {}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemInfo {
    pub cache_path: String,
    pub can_launch_web_browser: bool,
    pub can_self_restart: bool,
    pub completed_installations: Vec<String>,
    pub encoder_location: String,
    pub has_pending_restart: bool,
    pub has_update_available: bool,
    pub id: String,
    pub internal_metadata_path: String,
    pub is_shutting_down: bool,
    pub items_by_name_path: String,
    pub local_address: String,
    pub log_path: String,
    pub operating_system: String,
    pub operating_system_display_name: String,
    pub program_data_path: String,
    pub server_name: String,
    pub supports_library_monitor: bool,
    pub system_architecture: String,
    pub transcoding_temp_path: String,
    pub version: String,
    pub web_path: String,
    pub web_socket_port_number: u16,
}
