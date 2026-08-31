<div align="center">
  <br/>
  <p>
    <a href=https://github.com/caya8205-2/innertube-rs>
        <img src="site/public/innertube-rs-logo.png" alt="innertube-rs logo" width="200" />
    </a>
  </p>

  <p align="center">
   <strong>
    An asynchronous Rust client for <br>
    YouTube's internal API (InnerTube), ported from <a href="https://github.com/LuanRT/YouTube.js">YouTube.js</a>.
   </strong>
  </p>

  [![Crates](https://img.shields.io/crates/v/innertube-rs?style=flat?logo=rust&logoColor=white)](https://crates.io/crates/innertube-rs)
  [![Total Downloads](https://shields.io/crates/d/innertube-rs?style=flat?logo=rust&logoColor=white)](https://crates.io/crates/innertube-rs)
  [![Docs](https://docs.rs/innertube-rs/badge.svg)](https://docs.rs/innertube-rs)
  [![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![CI](https://github.com/caya8205-2/innertube-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/caya8205-2/innertube-rs/actions/workflows/ci.yml)

  <p align="center">
    <code>innertube-rs</code> provides native Rust bindings for interacting with <code>/youtubei/v1</code>, including metadata extraction, <br>
    signature deciphering, stream URL resolution, search, and channel/playlist scraping.
  </p>
</div>

## Features

- **Embedded Decipher Engine**: Uses an embedded QuickJS runtime (`rquickjs`) to execute player decipher routines (`sig` decipher and `n-token` transformation) in memory.
- **Async Runtime**: Built on `tokio` and `reqwest` with support for HTTP/2 multiplexing and native TLS.
- **Stream Extraction**: Resolves direct HTTPS stream URLs with configurable filters for audio-only and video formats.
- **InnerTube Endpoints**: Covers Player, Search, Browse (Channels, Playlists), YouTube Music, Comments, Live Chat, and Transcripts.
- **No External Subprocesses**: Operates as a single native binary without Node.js runtimes, Python interpreters, or `yt-dlp` subprocesses.

---

## Installation

```bash
cargo install innertube-rs
```
Running the above command will globally install the innertube binary.

### Install as library
Run the following Cargo command in your project directory:
```bash
cargo add innertube-rs
```

Or add the following line to your Cargo.toml:
```bash
innertube-rs = "0.8.0"
```

---

## Usage

### 1. Fetch Video Metadata and Resolve Stream URLs

```rust
use innertube_rs::{Innertube, FormatFilter, FormatType, QualityPreference};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;
    let video_id = "dQw4w9WgXcQ";

    // Fetch video metadata
    let info = yt.get_video_info(video_id).await?;
    if let Some(details) = info.video_details {
        println!("Title: {}", details.title);
        println!("Author: {}", details.author);
        println!("Duration: {}s", details.length_seconds);
    }

    // Resolve highest-quality audio stream
    let filter = FormatFilter {
        format_type: FormatType::AudioOnly,
        quality: QualityPreference::Highest,
        container: None,
    };

    let stream_url = yt.get_stream_url(video_id, &filter).await?;
    println!("Stream URL: {}", stream_url);

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
            SearchResultItem::Video(v) => println!("Video: {} ({})", v.title, v.video_id),
            SearchResultItem::Channel(c) => println!("Channel: {} ({})", c.title, c.channel_id),
            SearchResultItem::Playlist(p) => println!("Playlist: {} ({})", p.title, p.playlist_id),
        }
    }

    Ok(())
}
```

### 3. Fetch Channel Profile and Playlist

```rust
use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;

    // Channel metadata
    let channel = yt.get_channel("@RickAstleyYT").await?;
    println!("Channel: {}", channel.name);

    // Playlist items
    let playlist = yt.get_playlist("PLlaN88a7y2_plecYoJxeQNnWiiN01LUcZ").await?;
    println!("Playlist: {}", playlist.title);
    for video in playlist.videos {
        println!(" - {}", video.title);
    }

    Ok(())
}
```

---

## Examples

Run any of the included examples:

```bash
# Fetch video metadata and resolve playable stream URLs
cargo run --example get_video_info

# Search YouTube
cargo run --example search_and_browse

# Fetch YouTube Music artist data
cargo run --example get_music_artist

# Download audio stream to file
cargo run --example download_audio
```

---

## Testing

`innertube-rs` includes unit tests, deterministic fixture contract tests, live network integration tests, and authenticated lifecycle test suites.

### Standard Test Suites (Offline / Fixture-based)

```bash
# Run all unit tests, contract fixtures, and API contract test suites
cargo test --all-targets

# Run the central legacy AST renderer registry & parser contract tests
cargo test --lib parser::registry

# Run API contract suites matching YouTube.js public methods
cargo test --test api_contracts

# Run deterministic payload fixtures and AST dispatch tests
cargo test --test contract_fixtures

# Run protobuf encoder and manifest parser tests
cargo test utils::proto
cargo test utils::manifest
```

### Live Network Integration Tests

```bash
# Run live integration tests against YouTube's live servers (requires network)
cargo test --test live_integration -- --ignored

# Run live anonymous tests with detailed stdout logging
cargo test --test live_integration -- --ignored --nocapture
```

### Authenticated Mutation Integration Tests

```bash
# Run authenticated live tests (requires valid cookie and explicit opt-in)
INNERTUBE_COOKIE="your_cookie_here" INNERTUBE_MUTATION_TEST=1 cargo test --test authenticated_integration -- --ignored
```

### Code Quality & Static Analysis

```bash
# Run Clippy with strict warnings
cargo clippy --all-targets -- -D warnings
```

---

## Documentation

Full API reference is available at [docs.rs/innertube-rs](https://docs.rs/innertube-rs).

To generate local documentation:

```bash
cargo doc --no-deps --open
```

---

## License

[MIT](LICENSE)
