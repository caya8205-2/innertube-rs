use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerTubeContext {
    pub client: ClientContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<RequestContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientContext {
    pub hl: String,
    pub gl: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visitor_data: Option<String>,
    pub client_name: String,
    pub client_version: String,
    pub os_name: String,
    pub os_version: String,
    pub platform: String,
    pub client_form_factor: String,
    pub user_agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc_offset_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserContext {
    pub enable_safety_mode: bool,
    pub locked_safety_mode: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestContext {
    pub use_ssl: bool,
}
