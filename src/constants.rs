pub const YOUTUBE_BASE_URL: &str = "https://www.youtube.com";
pub const YOUTUBE_MUSIC_BASE_URL: &str = "https://music.youtube.com";
pub const INNERTUBE_API_BASE_URL: &str = "https://www.youtube.com/youtubei/v1";

pub const DEFAULT_INNERTUBE_KEY: &str = "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";
pub const DEFAULT_CLIENT_VERSION: &str = "2.20260820.08.00";
pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36";

pub const GOOGLE_SEARCH_BASE_URL: &str = "https://www.google.com/";

pub mod clients {
    pub const IOS_NAME: &str = "iOS";
    pub const IOS_VERSION: &str = "20.11.6";
    pub const IOS_USER_AGENT: &str = "com.google.ios.youtube/20.11.6 (iPhone10,4; U; CPU iOS 16_7_7 like Mac OS X)";
    pub const IOS_DEVICE_MODEL: &str = "iPhone10,4";
    pub const IOS_OS_NAME: &str = "iOS";
    pub const IOS_OS_VERSION: &str = "16.7.7.20H330";

    pub const ANDROID_NAME: &str = "ANDROID";
    pub const ANDROID_VERSION: &str = "21.03.36";
    pub const ANDROID_SDK_VERSION: u32 = 36;
    pub const ANDROID_USER_AGENT: &str = "com.google.android.youtube/21.03.36(Linux; U; Android 16; en_US; SM-S908E Build/TP1A.220624.014) gzip";

    pub const MWEB_NAME: &str = "MWEB";
    pub const MWEB_VERSION: &str = "2.20260205.04.01";
    pub const MWEB_USER_AGENT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 16_7_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1";

    pub const WEB_NAME: &str = "WEB";
    pub const WEB_VERSION: &str = "2.20260820.08.00";
    pub const WEB_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36";

    pub const WEB_KIDS_NAME: &str = "WEB_KIDS";
    pub const WEB_KIDS_VERSION: &str = "2.20260205.00.00";

    pub const WEB_REMIX_NAME: &str = "WEB_REMIX";
    pub const WEB_REMIX_VERSION: &str = "1.20250219.01.00";

    pub const ANDROID_VR_NAME: &str = "ANDROID_VR";
    pub const ANDROID_VR_VERSION: &str = "1.65.10";
    pub const ANDROID_VR_SDK_VERSION: u32 = 32;
    pub const ANDROID_VR_DEVICE_MAKE: &str = "Oculus";
    pub const ANDROID_VR_DEVICE_MODEL: &str = "Quest 3";
    pub const ANDROID_VR_USER_AGENT: &str = "com.google.android.apps.youtube.vr.oculus/1.65.10 (Linux; U; Android 12L; eureka-user Build/SQ3A.220605.009.A1) gzip";

    pub const VISIONOS_NAME: &str = "VISIONOS";
    pub const VISIONOS_VERSION: &str = "1.02";
    pub const VISIONOS_DEVICE_MAKE: &str = "Apple";
    pub const VISIONOS_DEVICE_MODEL: &str = "RealityDevice17,1";
    pub const VISIONOS_OS_NAME: &str = "visionOS";
    pub const VISIONOS_OS_VERSION: &str = "26.5.23O471";
    pub const VISIONOS_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15";

    pub const ANDROID_CREATOR_NAME: &str = "ANDROID_CREATOR";
    pub const ANDROID_CREATOR_VERSION: &str = "22.43.101";

    pub const ANDROID_MUSIC_NAME: &str = "ANDROID_MUSIC";
    pub const ANDROID_MUSIC_VERSION: &str = "5.34.51";

    pub const TV_NAME: &str = "TVHTML5";
    pub const TV_VERSION: &str = "7.20260311.12.00";
    pub const TV_USER_AGENT: &str = "Mozilla/5.0 (ChromiumStylePlatform) Cobalt/Version";

    pub const TV_SIMPLY_NAME: &str = "TVHTML5_SIMPLY";
    pub const TV_SIMPLY_VERSION: &str = "1.0";

    pub const TV_EMBEDDED_NAME: &str = "TVHTML5_SIMPLY_EMBEDDED_PLAYER";
    pub const TV_EMBEDDED_VERSION: &str = "2.0";

    pub const WEB_EMBEDDED_NAME: &str = "WEB_EMBEDDED_PLAYER";
    pub const WEB_EMBEDDED_VERSION: &str = "1.20260206.01.00";

    pub const WEB_CREATOR_NAME: &str = "WEB_CREATOR";
    pub const WEB_CREATOR_VERSION: &str = "1.20241203.01.00";
}

/// Client aliases accepted by `Session::adjust_context`, mirroring
/// `SUPPORTED_CLIENTS` in YouTube.js.
pub const SUPPORTED_CLIENTS: &[&str] = &[
    "IOS",
    "WEB",
    "MWEB",
    "YTKIDS",
    "YTMUSIC",
    "ANDROID",
    "ANDROID_VR",
    "VISIONOS",
    "YTSTUDIO_ANDROID",
    "YTMUSIC_ANDROID",
    "TV",
    "TV_SIMPLY",
    "TV_EMBEDDED",
    "WEB_EMBEDDED",
    "WEB_CREATOR",
];

