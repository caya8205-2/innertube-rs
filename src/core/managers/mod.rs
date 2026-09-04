pub mod account;
pub mod interaction;
pub mod kids;
pub mod music;
pub mod playlist;
pub mod studio;

pub use account::AccountManager;
pub use interaction::InteractionManager;
pub use kids::KidsManager;
pub use music::MusicManager;
pub use playlist::PlaylistManager;
pub use studio::{StudioManager, UpdateVideoMetadataOptions, UploadedVideoMetadataOptions, VideoPrivacy};
