use prost::Message;
use serde_json::{json, Value};

use crate::constants::clients;
use crate::core::actions::{Actions, ApiResponse};
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::proto::youtube::api::pfiinnertube::{
    metadata_update_request, InnerTubeContext, MetadataUpdateRequest,
};

const YT_UPLOAD_BASE: &str = "https://upload.youtube.com/";

/// Video privacy setting (legacy `UpdateVideoMetadataOptions.privacy`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoPrivacy {
    Public,
    Unlisted,
    Private,
}

impl VideoPrivacy {
    fn proto_value(self) -> i32 {
        match self {
            Self::Public => 1,
            Self::Unlisted => 2,
            Self::Private => 3,
        }
    }

    fn upload_value(self) -> &'static str {
        match self {
            Self::Public => "PUBLIC",
            Self::Unlisted => "UNLISTED",
            Self::Private => "PRIVATE",
        }
    }
}

/// Metadata fields for `update_video_metadata` (legacy
/// `UpdateVideoMetadataOptions`).
#[derive(Debug, Clone, Default)]
pub struct UpdateVideoMetadataOptions {
    pub title: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub tags: Option<Vec<String>>,
    pub thumbnail: Option<Vec<u8>>,
    pub category: Option<i32>,
    pub privacy: Option<VideoPrivacy>,
    pub made_for_kids: Option<bool>,
    pub age_restricted: Option<bool>,
}

/// Metadata for a new upload (legacy `UploadedVideoMetadataOptions`).
#[derive(Debug, Clone, Default)]
pub struct UploadedVideoMetadataOptions {
    pub title: Option<String>,
    pub description: Option<String>,
    pub privacy: Option<VideoPrivacy>,
    pub is_draft: Option<bool>,
}

struct InitialUploadData {
    frontend_upload_id: String,
    upload_url: String,
}

/// YouTube Studio client (1:1 with `core/clients/Studio.ts`). All methods
/// require authentication.
pub struct StudioManager<'a> {
    pub(crate) session: &'a Session,
}

impl<'a> StudioManager<'a> {
    pub fn new(session: &'a Session) -> Self {
        Self { session }
    }

    /// Update a video's metadata via a raw protobuf POST to
    /// `/video_manager/metadata_update`.
    pub async fn update_video_metadata(
        &self,
        video_id: &str,
        metadata: &UpdateVideoMetadataOptions,
    ) -> Result<ApiResponse> {
        self.session.ensure_authenticated()?;

        let payload = build_metadata_update_request(&self.session.context, video_id, metadata);
        let bytes = payload.encode_to_vec();

        Actions::execute(
            self.session,
            "/video_manager/metadata_update",
            json!({
                "protobuf": true,
                "serialized_data": bytes
            }),
        )
        .await
    }

    /// Upload a video using the legacy 3-phase resumable flow:
    /// start -> upload+finalize -> `/upload/createvideo`.
    pub async fn upload(
        &self,
        file: Vec<u8>,
        metadata: &UploadedVideoMetadataOptions,
    ) -> Result<ApiResponse> {
        self.session.ensure_authenticated()?;

        let initial = self.get_initial_upload_data().await?;
        let scotty_resource_id = self.upload_video(&initial.upload_url, file).await?;

        let payload = json!({
            "resourceId": {
                "scottyResourceId": {
                    "id": scotty_resource_id
                }
            },
            "frontendUploadId": initial.frontend_upload_id,
            "initialMetadata": {
                "title": {
                    "newTitle": metadata.title
                },
                "description": {
                    "newDescription": metadata.description,
                    "shouldSegment": true
                },
                "privacy": {
                    "newPrivacy": metadata.privacy.unwrap_or(VideoPrivacy::Private).upload_value()
                },
                "draftState": {
                    "isDraft": metadata.is_draft.unwrap_or(false)
                }
            }
        });

        Actions::execute(self.session, "/upload/createvideo", payload).await
    }

    async fn get_initial_upload_data(&self) -> Result<InitialUploadData> {
        let frontend_upload_id = format!(
            "innertube_android:{}:0:v=3,api=1,cf=3",
            crate::core::oauth::generate_uuid_v4()
        );

        let payload = json!({
            "frontendUploadId": frontend_upload_id,
            "deviceDisplayName": "Pixel 6 Pro",
            "fileId": format!(
                "goog-edited-video://generated?videoFileUri=content://media/external/video/media/{}",
                crate::core::oauth::generate_uuid_v4()
            ),
            "mp4MoovAtomRelocationStatus": "UNSUPPORTED",
            "transcodeResult": "DISABLED",
            "connectionType": "WIFI"
        });

        let resp = self
            .session
            .http_client
            .post(format!("{YT_UPLOAD_BASE}upload/youtubei"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("x-goog-upload-command", "start")
            .header("x-goog-upload-protocol", "resumable")
            .json(&payload)
            .send()
            .await
            .map_err(InnertubeError::Network)?;

        if !resp.status().is_success() {
            return Err(InnertubeError::Other(
                "Could not get initial upload data".to_string(),
            ));
        }

        let header = |name: &str| -> Result<String> {
            resp.headers()
                .get(name)
                .and_then(|v| v.to_str().ok())
                .map(ToString::to_string)
                .ok_or_else(|| {
                    InnertubeError::Other(format!("Missing upload response header: {name}"))
                })
        };

        Ok(InitialUploadData {
            frontend_upload_id,
            upload_url: header("x-goog-upload-url")?,
        })
    }

    async fn upload_video(&self, upload_url: &str, file: Vec<u8>) -> Result<String> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        let resp = self
            .session
            .http_client
            .post(upload_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("x-goog-upload-command", "upload, finalize")
            .header("x-goog-upload-file-name", format!("file-{timestamp}"))
            .header("x-goog-upload-offset", "0")
            .body(file)
            .send()
            .await
            .map_err(InnertubeError::Network)?;

        if !resp.status().is_success() {
            return Err(InnertubeError::Other("Could not upload video".to_string()));
        }

        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;
        let status = raw.get("status").and_then(Value::as_str).unwrap_or("");
        if status != "STATUS_SUCCESS" {
            return Err(InnertubeError::Other("Could not process video.".to_string()));
        }

        raw.get("scottyResourceId")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .ok_or_else(|| InnertubeError::Other("Missing scottyResourceId".to_string()))
    }
}

/// Build the legacy `MetadataUpdateRequest` protobuf (hardcoded Android
/// client context, per `Studio.updateVideoMetadata`).
fn build_metadata_update_request(
    context: &crate::models::context::InnerTubeContext,
    video_id: &str,
    metadata: &UpdateVideoMetadataOptions,
) -> MetadataUpdateRequest {
    use crate::proto::youtube::api::pfiinnertube::client_info::ConfigGroupsClientInfo;
    use crate::proto::youtube::api::pfiinnertube::ClientInfo;

    let mut payload = MetadataUpdateRequest {
        context: Some(InnerTubeContext {
            client: Some(ClientInfo {
                os_name: Some("Android".to_string()),
                client_name: Some(3), // CLIENT_NAME_IDS.ANDROID
                client_version: Some(clients::ANDROID_VERSION.to_string()),
                android_sdk_version: Some(clients::ANDROID_SDK_VERSION as i32),
                visitor_data: context.client.visitor_data.clone(),
                os_version: Some("13".to_string()),
                accept_language: Some(context.client.hl.clone()),
                accept_region: Some(context.client.gl.clone()),
                device_make: Some("Google".to_string()),
                device_model: Some("sdk_gphone64_x86_64".to_string()),
                screen_height_points: Some(840),
                screen_width_points: Some(432),
                config_info: Some(ConfigGroupsClientInfo {
                    app_install_data: context
                        .client
                        .config_info
                        .as_ref()
                        .and_then(|c| c.app_install_data.clone()),
                    ..Default::default()
                }),
                time_zone: context.client.time_zone.clone(),
                chipset: Some("qcom;taro".to_string()),
                ..Default::default()
            }),
            active_players: Vec::new(),
            ..Default::default()
        }),
        encrypted_video_id: Some(video_id.to_string()),
        ..Default::default()
    };

    if let Some(ref title) = metadata.title {
        payload.title = Some(metadata_update_request::MdeTitleUpdateRequest {
            new_title: Some(title.clone()),
        });
    }
    if let Some(ref description) = metadata.description {
        payload.description = Some(metadata_update_request::MdeDescriptionUpdateRequest {
            new_description: Some(description.clone()),
        });
    }
    if let Some(ref license) = metadata.license {
        payload.license = Some(metadata_update_request::MdeLicenseUpdateRequest {
            new_license_id: Some(license.clone()),
        });
    }
    if let Some(ref tags) = metadata.tags {
        payload.tags = Some(metadata_update_request::MdeTagsUpdateRequest {
            new_tags: tags.clone(),
        });
    }
    if let Some(ref thumbnail) = metadata.thumbnail {
        payload.video_still = Some(metadata_update_request::MdeVideoStillRequestParams {
            operation: Some(3),
            new_still_id: None,
            image: Some(
                metadata_update_request::mde_video_still_request_params::CustomThumbnailImage {
                    raw_bytes: Some(thumbnail.clone()),
                    ..Default::default()
                },
            ),
            test_image: None,
            experiment_image: Vec::new(),
        });
    }
    if let Some(category) = metadata.category {
        payload.category = Some(metadata_update_request::MdeCategoryUpdateRequest {
            new_category_id: Some(category),
        });
    }
    if let Some(privacy) = metadata.privacy {
        payload.privacy = Some(metadata_update_request::MdePrivacyUpdateRequest {
            new_privacy: Some(privacy.proto_value()),
            clear_privacy_draft: None,
        });
    }
    if let Some(made_for_kids) = metadata.made_for_kids {
        payload.made_for_kids = Some(
            metadata_update_request::MdeMadeForKidsUpdateRequestParams {
                operation: Some(1),
                new_mfk: Some(if made_for_kids { 1 } else { 2 }),
            },
        );
    }
    if let Some(age_restricted) = metadata.age_restricted {
        payload.racy = Some(metadata_update_request::MdeRacyRequestParams {
            operation: Some(1),
            new_racy: Some(if age_restricted { 1 } else { 2 }),
        });
    }

    payload
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::session::SessionOptions;

    #[test]
    fn metadata_update_request_matches_legacy_shape() {
        let context = Session::build_default_context(&SessionOptions {
            lang: Some("en".to_string()),
            location: Some("US".to_string()),
            visitor_data: Some("visitor-data".to_string()),
            ..Default::default()
        });

        let metadata = UpdateVideoMetadataOptions {
            title: Some("New Title".to_string()),
            description: Some("New Description".to_string()),
            license: Some("creative_commons".to_string()),
            tags: Some(vec!["a".to_string(), "b".to_string()]),
            thumbnail: Some(vec![1, 2, 3]),
            category: Some(27),
            privacy: Some(VideoPrivacy::Unlisted),
            made_for_kids: Some(false),
            age_restricted: Some(true),
        };

        let req = build_metadata_update_request(&context, "vid123", &metadata);

        assert_eq!(req.encrypted_video_id.as_deref(), Some("vid123"));
        let client = req.context.as_ref().unwrap().client.as_ref().unwrap();
        assert_eq!(client.client_name, Some(3));
        assert_eq!(client.client_version.as_deref(), Some(clients::ANDROID_VERSION));
        assert_eq!(client.android_sdk_version, Some(36));
        assert_eq!(client.visitor_data.as_deref(), Some("visitor-data"));
        assert_eq!(client.os_name.as_deref(), Some("Android"));
        assert_eq!(client.os_version.as_deref(), Some("13"));
        assert_eq!(client.device_make.as_deref(), Some("Google"));
        assert_eq!(client.device_model.as_deref(), Some("sdk_gphone64_x86_64"));
        assert_eq!(client.screen_height_points, Some(840));
        assert_eq!(client.screen_width_points, Some(432));
        assert_eq!(client.chipset.as_deref(), Some("qcom;taro"));
        assert_eq!(client.accept_language.as_deref(), Some("en"));
        assert_eq!(client.accept_region.as_deref(), Some("US"));

        assert_eq!(
            req.title.as_ref().unwrap().new_title.as_deref(),
            Some("New Title")
        );
        assert_eq!(req.privacy.as_ref().unwrap().new_privacy, Some(2));
        assert_eq!(req.category.as_ref().unwrap().new_category_id, Some(27));
        assert_eq!(
            req.tags.as_ref().unwrap().new_tags,
            vec!["a".to_string(), "b".to_string()]
        );
        let still = req.video_still.as_ref().unwrap();
        assert_eq!(still.operation, Some(3));
        assert_eq!(
            still.image.as_ref().unwrap().raw_bytes.as_deref(),
            Some(&[1u8, 2, 3][..])
        );
        assert_eq!(req.made_for_kids.as_ref().unwrap().new_mfk, Some(2));
        assert_eq!(req.racy.as_ref().unwrap().new_racy, Some(1));

        // Must encode/decode losslessly.
        let bytes = req.encode_to_vec();
        let decoded = MetadataUpdateRequest::decode(&bytes[..]).unwrap();
        assert_eq!(decoded.encrypted_video_id.as_deref(), Some("vid123"));
    }

    #[test]
    fn privacy_mapping_matches_legacy() {
        assert_eq!(VideoPrivacy::Public.proto_value(), 1);
        assert_eq!(VideoPrivacy::Unlisted.proto_value(), 2);
        assert_eq!(VideoPrivacy::Private.proto_value(), 3);
    }
}
