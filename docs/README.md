# innertube-rs Documentation

Welcome to the documentation for **`innertube-rs`**, a high-performance, asynchronous pure Rust client for YouTube's internal API (InnerTube).

---

## 📚 Documentation Index

1. **[Architecture & Internal Design](architecture.md)**
   - Overview of the client architecture, session management, and HTTP client.
2. **[Innertube Client API](api/core/innertube.md)**
   - Complete reference for top-level methods on the `Innertube` struct.
3. **[Session & Authentication](api/core/session.md)**
   - Visitor data protobuf generation, PO-token passing, and multi-client contextual headers.
4. **[Player & Decipher Engine](api/core/player.md)**
   - Embedded QuickJS sandbox (`rquickjs`) executing signature decipher algorithms and n-token transformations.
5. **[OAuth2 Device Flow](api/core/oauth.md)**
   - Google TV / Device authorization flow and automatic token refreshing.
6. **[Account Mutation Actions](api/core/actions.md)**
   - Like, Dislike, Subscribe, Unsubscribe, Playlist Management, and Commenting.
7. **[API & Endpoints Reference](api/README.md)**
   - Complete index of all endpoints, models, and AST nodes.
8. **[Modular AST Component Parser](updating-the-parser.md)**
   - Technical guide to the recursive polymorphic AST parser (`src/parser/nodes/`).

---

## ⚡ Quick Start Example

```rust
use innertube_rs::{Innertube, FormatFilter, FormatType, QualityPreference};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize client
    let yt = Innertube::new().await?;

    // 2. Fetch video metadata
    let info = yt.get_video_info("dQw4w9WgXcQ").await?;
    if let Some(details) = info.video_details {
        println!("Title: {}", details.title);
        println!("Author: {}", details.author);
        println!("Duration: {}s", details.length_seconds);
    }

    // 3. Resolve direct audio stream
    let filter = FormatFilter {
        format_type: FormatType::AudioOnly,
        quality: QualityPreference::Highest,
        container: None,
    };
    let stream_url = yt.get_stream_url("dQw4w9WgXcQ", &filter).await?;
    println!("Stream URL: {}", stream_url);

    Ok(())
}
```
