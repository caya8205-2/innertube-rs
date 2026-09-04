//! Contract tests for per-client context adjustment (Batch 1), mirroring
//! `HTTPClient.#adjustContext` and `Session.#buildContext` in YouTube.js.

use innertube_rs::constants::clients;
use innertube_rs::models::context::InnerTubeContext;
use innertube_rs::{Session, SessionOptions};

fn base_context() -> InnerTubeContext {
    Session::build_default_context(&SessionOptions::default())
}

fn adjusted(client: &str) -> InnerTubeContext {
    let mut ctx = base_context();
    Session::adjust_context(&mut ctx, client).expect("supported client should adjust");
    ctx
}

#[test]
fn default_context_matches_legacy_build_context() {
    let ctx = base_context();
    let c = &ctx.client;
    assert_eq!(c.screen_density_float, Some(1));
    assert_eq!(c.screen_height_points, Some(1440));
    assert_eq!(c.screen_pixel_density, Some(1));
    assert_eq!(c.screen_width_points, Some(2560));
    assert_eq!(
        c.user_interface_theme.as_deref(),
        Some("USER_INTERFACE_THEME_LIGHT")
    );
    assert_eq!(c.original_url.as_deref(), Some("https://www.youtube.com"));
    assert_eq!(c.memory_total_kbytes.as_deref(), Some("8000000"));
    let main = c.main_app_web_info.as_ref().expect("mainAppWebInfo");
    assert_eq!(main.graft_url, "https://www.youtube.com");
    assert_eq!(
        main.pwa_installability_status,
        "PWA_INSTALLABILITY_STATUS_UNKNOWN"
    );
    assert_eq!(main.web_display_mode, "WEB_DISPLAY_MODE_BROWSER");
    assert!(main.is_web_native_share_available);
    let req = ctx.request.as_ref().expect("request context");
    assert!(req.use_ssl);
    assert_eq!(req.internal_experiment_flags, Some(vec![]));
}

#[test]
fn mweb_adjustment_sets_mobile_small_form_factor() {
    let ctx = adjusted("MWEB");
    assert_eq!(ctx.client.client_name, clients::MWEB_NAME);
    assert_eq!(ctx.client.client_version, clients::MWEB_VERSION);
    assert_eq!(ctx.client.client_form_factor, "SMALL_FORM_FACTOR");
    assert_eq!(ctx.client.platform, "MOBILE");
}

#[test]
fn ios_adjustment_sets_device_and_strips_browser() {
    let mut ctx = base_context();
    ctx.client.browser_name = Some("Chrome".to_string());
    ctx.client.browser_version = Some("125.0.0.0".to_string());
    Session::adjust_context(&mut ctx, "IOS").unwrap();
    assert_eq!(ctx.client.client_name, clients::IOS_NAME);
    assert_eq!(ctx.client.client_version, clients::IOS_VERSION);
    assert_eq!(ctx.client.device_make.as_deref(), Some("Apple"));
    assert_eq!(ctx.client.device_model.as_deref(), Some(clients::IOS_DEVICE_MODEL));
    assert_eq!(ctx.client.os_name, clients::IOS_OS_NAME);
    assert_eq!(ctx.client.os_version, clients::IOS_OS_VERSION);
    assert_eq!(ctx.client.platform, "MOBILE");
    assert!(ctx.client.browser_name.is_none());
    assert!(ctx.client.browser_version.is_none());
}

#[test]
fn ytmusic_adjustment_maps_to_web_remix() {
    let ctx = adjusted("YTMUSIC");
    assert_eq!(ctx.client.client_name, clients::WEB_REMIX_NAME);
    assert_eq!(ctx.client.client_version, clients::WEB_REMIX_VERSION);
}

#[test]
fn android_adjustment_sets_sdk_os_and_mobile() {
    let ctx = adjusted("ANDROID");
    assert_eq!(ctx.client.client_name, clients::ANDROID_NAME);
    assert_eq!(ctx.client.client_version, clients::ANDROID_VERSION);
    assert_eq!(
        ctx.client.android_sdk_version,
        Some(clients::ANDROID_SDK_VERSION)
    );
    assert_eq!(ctx.client.os_name, "Android");
    assert_eq!(ctx.client.os_version, "13");
    assert_eq!(ctx.client.platform, "MOBILE");
    assert_eq!(ctx.client.user_agent, clients::ANDROID_USER_AGENT);
    assert_eq!(ctx.client.client_form_factor, "SMALL_FORM_FACTOR");
}

#[test]
fn android_vr_adjustment_sets_oculus_device() {
    let ctx = adjusted("ANDROID_VR");
    assert_eq!(ctx.client.client_name, clients::ANDROID_VR_NAME);
    assert_eq!(ctx.client.client_version, clients::ANDROID_VR_VERSION);
    assert_eq!(
        ctx.client.android_sdk_version,
        Some(clients::ANDROID_VR_SDK_VERSION)
    );
    assert_eq!(ctx.client.os_version, "12L");
    assert_eq!(ctx.client.device_make.as_deref(), Some("Oculus"));
    assert_eq!(ctx.client.device_model.as_deref(), Some("Quest 3"));
    assert_eq!(ctx.client.user_agent, clients::ANDROID_VR_USER_AGENT);
}

#[test]
fn visionos_adjustment_strips_browser_and_sets_device() {
    let ctx = adjusted("VISIONOS");
    assert_eq!(ctx.client.client_name, clients::VISIONOS_NAME);
    assert_eq!(ctx.client.client_version, clients::VISIONOS_VERSION);
    assert_eq!(
        ctx.client.device_model.as_deref(),
        Some(clients::VISIONOS_DEVICE_MODEL)
    );
    assert_eq!(ctx.client.os_name, clients::VISIONOS_OS_NAME);
    assert!(ctx.client.browser_name.is_none());
    assert!(ctx.client.browser_version.is_none());
}

#[test]
fn ytmusic_android_and_studio_share_android_base() {
    for (alias, name, version) in [
        ("YTMUSIC_ANDROID", clients::ANDROID_MUSIC_NAME, clients::ANDROID_MUSIC_VERSION),
        ("YTSTUDIO_ANDROID", clients::ANDROID_CREATOR_NAME, clients::ANDROID_CREATOR_VERSION),
    ] {
        let ctx = adjusted(alias);
        assert_eq!(ctx.client.client_name, name, "{alias}");
        assert_eq!(ctx.client.client_version, version, "{alias}");
        assert_eq!(
            ctx.client.android_sdk_version,
            Some(clients::ANDROID_SDK_VERSION),
            "{alias}"
        );
        assert_eq!(ctx.client.os_name, "Android", "{alias}");
        assert_eq!(ctx.client.platform, "MOBILE", "{alias}");
    }
}

#[test]
fn tv_adjustments_set_versions() {
    let tv = adjusted("TV");
    assert_eq!(tv.client.client_name, clients::TV_NAME);
    assert_eq!(tv.client.client_version, clients::TV_VERSION);
    assert_eq!(tv.client.user_agent, clients::TV_USER_AGENT);

    let simply = adjusted("TV_SIMPLY");
    assert_eq!(simply.client.client_name, clients::TV_SIMPLY_NAME);
    assert_eq!(simply.client.client_version, clients::TV_SIMPLY_VERSION);
}

#[test]
fn tv_embedded_sets_embed_screen_and_youtube_third_party() {
    let ctx = adjusted("TV_EMBEDDED");
    assert_eq!(ctx.client.client_name, clients::TV_EMBEDDED_NAME);
    assert_eq!(ctx.client.client_version, clients::TV_EMBEDDED_VERSION);
    assert_eq!(ctx.client.client_screen.as_deref(), Some("EMBED"));
    assert_eq!(
        ctx.third_party.as_ref().map(|t| t.embed_url.as_str()),
        Some("https://www.youtube.com")
    );
}

#[test]
fn web_embedded_sets_google_search_embed_url() {
    let ctx = adjusted("WEB_EMBEDDED");
    assert_eq!(ctx.client.client_name, clients::WEB_EMBEDDED_NAME);
    assert_eq!(ctx.client.client_version, clients::WEB_EMBEDDED_VERSION);
    assert_eq!(ctx.client.client_screen.as_deref(), Some("EMBED"));
    assert_eq!(
        ctx.third_party.as_ref().map(|t| t.embed_url.as_str()),
        Some("https://www.google.com/")
    );
}

#[test]
fn web_creator_adjustment() {
    let ctx = adjusted("WEB_CREATOR");
    assert_eq!(ctx.client.client_name, clients::WEB_CREATOR_NAME);
    assert_eq!(ctx.client.client_version, clients::WEB_CREATOR_VERSION);
}

#[test]
fn ytkids_adjustment_adds_kids_app_info() {
    let ctx = adjusted("YTKIDS");
    assert_eq!(ctx.client.client_name, clients::WEB_KIDS_NAME);
    assert_eq!(ctx.client.client_version, clients::WEB_KIDS_VERSION);
    let info = ctx.client.kids_app_info.as_ref().expect("kidsAppInfo");
    assert_eq!(info.category_settings.enabled_categories.len(), 20);
    assert!(info
        .category_settings
        .enabled_categories
        .contains(&"approved_for_you".to_string()));
    assert_eq!(
        info.content_settings.corpus_preference,
        "KIDS_CORPUS_PREFERENCE_YOUNGER"
    );
    assert_eq!(
        info.content_settings.kids_no_search_mode,
        "YT_KIDS_NO_SEARCH_MODE_OFF"
    );
}

#[test]
fn config_info_removed_for_non_web_clients() {
    let mut ctx = base_context();
    ctx.client.config_info = Some(Default::default());
    Session::adjust_context(&mut ctx, "ANDROID").unwrap();
    assert!(ctx.client.config_info.is_none());

    let mut web_ctx = base_context();
    web_ctx.client.config_info = Some(Default::default());
    Session::adjust_context(&mut web_ctx, "WEB").unwrap();
    assert!(web_ctx.client.config_info.is_some());
}

#[test]
fn invalid_client_lists_supported_clients() {
    let mut ctx = base_context();
    let err = Session::adjust_context(&mut ctx, "NOT_A_CLIENT").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Invalid client: NOT_A_CLIENT"), "{msg}");
    assert!(msg.contains("YTMUSIC"), "{msg}");
    assert!(msg.contains("WEB_CREATOR"), "{msg}");
}

#[test]
fn client_name_id_matches_legacy_map() {
    assert_eq!(Session::client_name_id("WEB"), "1");
    assert_eq!(Session::client_name_id("MWEB"), "2");
    assert_eq!(Session::client_name_id("ANDROID"), "3");
    assert_eq!(Session::client_name_id("iOS"), "5");
    assert_eq!(Session::client_name_id("TVHTML5"), "7");
    assert_eq!(Session::client_name_id("ANDROID_CREATOR"), "14");
    assert_eq!(Session::client_name_id("ANDROID_MUSIC"), "21");
    assert_eq!(Session::client_name_id("ANDROID_VR"), "28");
    assert_eq!(Session::client_name_id("WEB_EMBEDDED_PLAYER"), "56");
    assert_eq!(Session::client_name_id("WEB_CREATOR"), "62");
    assert_eq!(Session::client_name_id("WEB_REMIX"), "67");
    assert_eq!(Session::client_name_id("TVHTML5_SIMPLY"), "74");
    assert_eq!(Session::client_name_id("WEB_KIDS"), "76");
    assert_eq!(
        Session::client_name_id("TVHTML5_SIMPLY_EMBEDDED_PLAYER"),
        "85"
    );
    assert_eq!(Session::client_name_id("VISIONOS"), "101");
}
