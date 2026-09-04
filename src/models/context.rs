use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerTubeContext {
    pub client: ClientContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<RequestContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party: Option<ThirdPartyContext>,
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
    pub android_sdk_version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_make: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc_offset_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_density_float: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_height_points: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_pixel_density: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_width_points: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_interface_theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_total_kbytes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollout_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_experiment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_screen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_app_web_info: Option<MainAppWebInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_info: Option<ConfigInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kids_app_info: Option<KidsAppInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainAppWebInfo {
    pub graft_url: String,
    pub pwa_installability_status: String,
    pub web_display_mode: String,
    pub is_web_native_share_available: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_install_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cold_config_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cold_hash_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hot_hash_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KidsAppInfo {
    pub category_settings: KidsCategorySettings,
    pub content_settings: KidsContentSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KidsCategorySettings {
    pub enabled_categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KidsContentSettings {
    pub corpus_preference: String,
    pub kids_no_search_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThirdPartyContext {
    pub embed_url: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserContext {
    pub enable_safety_mode: bool,
    pub locked_safety_mode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of_user: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestContext {
    pub use_ssl: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_experiment_flags: Option<Vec<serde_json::Value>>,
}
