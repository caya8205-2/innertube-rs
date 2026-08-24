# innertube-rs

[![Crates.io](https://img.shields.io/crates/v/innertube-rs.svg)](https://crates.io/crates/innertube-rs)
[![Documentation](https://docs.rs/innertube-rs/badge.svg)](https://docs.rs/innertube-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/caya8205-2/innertube-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/caya8205-2/innertube-rs/actions/workflows/ci.yml)

A fast, lightweight, and asynchronous pure **Rust port of [YouTube.js (InnerTube)](https://github.com/LuanRT/YouTube.js)**.

`innertube-rs` provides native Rust bindings for interacting with YouTube's private internal API (`/youtubei/v1`), including metadata extraction, signature deciphering, stream URL resolution, search, and channel/playlist scraping.

---

## ✨ Features

- ⚡ **Pure Rust & Async**: Built on `tokio` and `reqwest` for maximum concurrency and ultra-low memory footprint.
- 🔓 **Embedded Decipher Engine**: Uses an embedded QuickJS sandbox (`rquickjs`) to execute YouTube's player decipher routines (`sig` decipher + `n-token` transformation) in `<5ms`.
- 🎵 **Adaptive Stream Resolution**: Easily extract and filter direct audio-only (`Opus`, `AAC`) and video (`1080p`, `4K`) streams without external tools like `yt-dlp`.
- 🔍 **Search & Scraping**: Full support for video/channel/playlist search and channel scraping compatible with modern YouTube schemas.
- 📦 **Zero Runtime Overhead**: No Node.js runtime, no Python subprocesses, and no bloated sidecars.

---

## 📦 Installation

Add `innertube-rs` to your `Cargo.toml`:

```toml
[dependencies]
innertube-rs = "0.5"
tokio = { version = "1", features = ["full"] }
```

---

## 🚀 Usage

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

### 2. Search Videos, Channels, and Playlists

```rust
use innertube_rs::{Innertube, SearchResultItem};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;
    let results = yt.search("rick astley never gonna give you up", None).await?;

    for item in results.items {
        match item {
            SearchResultItem::Video(v) => println!("Video: {} (ID: {})", v.title, v.video_id),
            SearchResultItem::Channel(c) => println!("Channel: {} (ID: {})", c.title, c.channel_id),
            SearchResultItem::Playlist(p) => println!("Playlist: {} (ID: {})", p.title, p.playlist_id),
        }
    }

    Ok(())
}
```

### 3. Fetch Channel Profile & Playlist Tracklist

```rust
use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;

    // Channel metadata & top tracks
    let channel = yt.get_channel("@RickAstleyYT").await?;
    println!("Channel: {} (Followers: {:?})", channel.name, channel.followers);
    println!("Top tracks count: {}", channel.top_tracks.len());

    // Playlist tracks
    let playlist = yt.get_playlist("PLlaN88a7y2_plecYoJxeQNnWiiN01LUcZ").await?;
    println!("Playlist: {}", playlist.title);
    for video in playlist.videos {
        println!(" - {} by {:?}", video.title, video.author.map(|a| a.name));
    }

    Ok(())
}
```

---

## 🏃 Examples

Run any of the built-in examples directly:

```bash
# Test Session bootstrap & QuickJS decipher engine
cargo run --example test_session

# Fetch video metadata and resolve direct playable stream URLs
cargo run --example get_video_info

# Search and browse channels / playlists
cargo run --example search_and_browse

# Download an audio stream chunk to a local file (.m4a)
cargo run --example download_audio
```

---

## 📖 Generating Documentation

To build and view the local HTML documentation site:

```bash
cargo doc --no-deps --open
```

---

## 📄 License

MIT License. See [LICENSE](LICENSE) for details.
