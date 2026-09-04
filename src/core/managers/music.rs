use crate::core::session::Session;
use crate::endpoints::music::{
    get_music_album, get_music_artist, get_music_explore, get_music_home, get_music_lyrics,
    search_music,
};
use crate::error::Result;
use crate::models::music::{
    MusicAlbumView, MusicArtistPage, MusicExplore, MusicHomeFeed, MusicLyrics, MusicSearchFilter,
    MusicSearchResults,
};

/// YouTube Music Manager (1:1 with Music.ts).
pub struct MusicManager<'a> {
    pub(crate) session: &'a Session,
}

impl<'a> MusicManager<'a> {
    pub fn new(session: &'a Session) -> Self {
        Self { session }
    }

    /// Search YouTube Music.
    pub async fn search(
        &self,
        query: &str,
        filter: Option<MusicSearchFilter>,
    ) -> Result<MusicSearchResults> {
        search_music(self.session, query, filter).await
    }

    /// Fetch a YouTube Music artist page.
    pub async fn get_artist(&self, artist_id: &str) -> Result<MusicArtistPage> {
        get_music_artist(self.session, artist_id).await
    }

    /// Fetch a YouTube Music album page.
    pub async fn get_album(&self, album_id: &str) -> Result<MusicAlbumView> {
        get_music_album(self.session, album_id).await
    }

    /// Fetch song lyrics.
    pub async fn get_lyrics(&self, video_id: &str) -> Result<MusicLyrics> {
        get_music_lyrics(self.session, video_id).await
    }

    /// Fetch YouTube Music Home Feed.
    pub async fn get_home(&self) -> Result<MusicHomeFeed> {
        get_music_home(self.session).await
    }

    /// Fetch YouTube Music Explore page.
    pub async fn get_explore(&self) -> Result<MusicExplore> {
        get_music_explore(self.session).await
    }

    /// Fetch a YouTube Music playlist (legacy `Music.getPlaylist`).
    pub async fn get_playlist(
        &self,
        playlist_id: &str,
    ) -> Result<crate::models::playlist::PlaylistView> {
        crate::endpoints::music::get_music_playlist(self.session, playlist_id).await
    }

    /// Fetch the YouTube Music library landing page (legacy `Music.getLibrary`).
    pub async fn get_library(&self) -> Result<Vec<crate::parser::YTNode>> {
        crate::endpoints::music::get_music_library(self.session).await
    }

    /// Fetch the listening-review recap (legacy `Music.getRecap`).
    pub async fn get_recap(&self) -> Result<Vec<crate::parser::YTNode>> {
        crate::endpoints::music::get_music_recap(self.session).await
    }

    /// Fetch the watch-next queue panel, following automix (legacy
    /// `Music.getUpNext`).
    pub async fn get_up_next(
        &self,
        video_id: &str,
        automix: bool,
    ) -> Result<crate::parser::nodes::playlist::PlaylistPanelNode> {
        crate::endpoints::music::get_music_up_next(self.session, video_id, automix).await
    }

    /// Fetch related tracks (legacy `Music.getRelated`).
    pub async fn get_related(&self, video_id: &str) -> Result<Vec<crate::parser::YTNode>> {
        crate::endpoints::music::get_music_related(self.session, video_id).await
    }

    /// Fetch YouTube Music search suggestions (legacy
    /// `Music.getSearchSuggestions`).
    pub async fn get_search_suggestions(
        &self,
        input: &str,
    ) -> Result<crate::models::suggestions::SearchSuggestionsResult> {
        crate::endpoints::suggestions::get_search_suggestions(self.session, input, true).await
    }
}
