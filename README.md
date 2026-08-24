# innertube-rs

[![Rust](https://img.shields.io/badge/rust-2021%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Release](https://img.shields.io/github/v/release/caya8205-2/innertube-rs)](https://github.com/caya8205-2/innertube-rs/releases)

A high-performance, asynchronous, pure **Rust port of [YouTube.js (InnerTube)](https://github.com/LuanRT/YouTube.js)**.

`innertube-rs` provides native Rust bindings for interacting with YouTube's private internal API (`/youtubei/v1`), including media streaming, deciphering signatures & n-tokens via an embedded QuickJS engine, YouTube Music extraction, video recommendations, playlists, comments, transcripts, and channel scrapers.

---

## ✨ Features

- ⚡ **Pure Rust & Async**: Built on `tokio` and `reqwest` for maximum concurrency and ultra-low memory footprint.
- 🔓 **Embedded Decipher Engine**: Uses an embedded QuickJS sandbox (`rquickjs`) to execute YouTube's player decipher routines (`sig` decipher + `n-token` transformation) in `<5ms`.
- 🎵 **Adaptive Stream Resolution**: Easily extract and filter direct audio-only (`Opus`, `AAC`) and video (`1080p`, `4K`) streams without external tools like `yt-dlp`.
- 🎧 **YouTube Music (`WEB_REMIX`) Suite**: Filtered search (Songs, Albums, Artists, Playlists), album tracklists, song lyrics (LyricFind / Musixmatch), and Explore / Trending charts.
- 📜 **Subtitles & Transcripts**: Timed transcript JSON3 & XML parser with built-in export to SubRip (`.srt`) and WebVTT (`.vtt`).
- 💬 **Comments & Threads**: Top comments, pinned comments, author creator badges, likes count, and reply threads.
- 📑 **Playlists & Continuations**: Full YouTube playlist scraping supporting both modern `lockupViewModel` and continuation pagination.
- 👤 **Channel Extended Tabs**: Uploaded Videos, YouTube Shorts, and Channel About profile metadata.
- 📦 **Zero Runtime Overhead**: No Node.js runtime, no Python subprocesses, and no bloated sidecars.

---

## 📦 Installation

Add `innertube-rs` to your `Cargo.toml`:

```toml
[dependencies]
innertube-rs = { git = "https://github.com/caya8205-2/innertube-rs" }
tokio = { version = "1", features = ["full"] }
```

---

## 🚀 Quick Start

### 1. Initialize Client & Fetch Video Info

```rust
use innertube_rs::{Innertube, FormatFilter, FormatType, QualityPreference};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client (bootstraps session and decipher engine)
    let yt = Innertube::new().await?;

    let video_id = "dQw4w9WgXcQ";

    // 1. Fetch metadata
    let info = yt.get_video_info(video_id).await?;
    if let Some(details) = info.video_details {
        println!("Title: {}", details.title);
        println!("Author: {}", details.author);
        println!("Duration: {}s", details.length_seconds);
    }

    // 2. Resolve highest quality audio-only stream URL
    let filter = FormatFilter {
        format_type: FormatType::AudioOnly,
        quality: QualityPreference::Highest,
        container: None,
    };

    let stream_url = yt.get_stream_url(video_id, &filter).await?;
    println!("Direct Audio Stream URL: {}", stream_url);

    Ok(())
}
```

### 2. YouTube Music Search & Lyrics

```rust
use innertube_rs::{Innertube, MusicSearchFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;

    // Search songs on YouTube Music
    let results = yt.search_music("yoasobi idol", Some(MusicSearchFilter::Songs)).await?;
    for song in results.songs.iter().take(3) {
        println!("Song: {} - {} [{}] (ID: {})", song.title, song.artist, song.duration, song.id);
    }

    // Fetch lyrics
    let lyrics = yt.get_music_lyrics("m9SMT5ipbxk").await?;
    println!("Lyrics Source: {:?}", lyrics.source);
    println!("Lyrics:\n{}", lyrics.text);

    Ok(())
}
```

### 3. Autocomplete Search Suggestions

```rust
use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;

    // Standard YouTube Suggestions
    let yt_sug = yt.get_search_suggestions("rick astley", false).await?;
    println!("YouTube Suggestions: {:?}", yt_sug.suggestions);

    // YouTube Music Suggestions
    let music_sug = yt.get_search_suggestions("yoasobi", true).await?;
    println!("YT Music Suggestions: {:?}", music_sug.suggestions);

    Ok(())
}
```

---

## 🏃 Runnable Examples

The repository includes standalone diagnostic examples in `examples/`:

```bash
# Autocomplete search suggestions
cargo run --example get_suggestions -- "yoasobi"

# Full YouTube playlist scraper
cargo run --example get_playlist -- "PLcwSN1dMiPp7-zS0pz_uiRreJopx9EHpY"

# Channel videos, shorts, and profile about
cargo run --example get_channel_tabs -- "UCX6OQ3DkcsbYNE6H8uQQuVA"

# YouTube Music search, albums, lyrics, and explore charts
cargo run --example test_music_search -- "yoasobi"
cargo run --example get_music_album -- "MPREb_kS20230601"
cargo run --example get_music_lyrics -- "m9SMT5ipbxk"
cargo run --example get_music_explore

# Timed subtitles and export to SRT / VTT
cargo run --example get_transcript -- "dQw4w9WgXcQ"

# Video comments and reply threads
cargo run --example get_comments -- "dQw4w9WgXcQ"

# Multi-client fallback streaming test
cargo run --example test_clients -- "dQw4w9WgXcQ"
```

---

## 📖 Upstream Reference

Upstream TypeScript reference files from `YouTube.js` are preserved in the `reference-youtubejs` branch for future hardening and feature parity comparisons:

```bash
git checkout reference-youtubejs
```

---

## 📄 License

MIT License. See [LICENSE](LICENSE) for details.
