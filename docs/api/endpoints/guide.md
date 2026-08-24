# Endpoint: Guide (`src/endpoints/guide.rs`)

Fetches the navigation menu items, subscriptions, and shortcuts from `/guide`.

```rust
use innertube_rs::endpoints::guide::get_guide;
```

---

## Functions

### `get_guide(session: &Session) -> Result<GuideResponse>`
Fetches sidebar guide sections, channel subscriptions, library links, and category shortcuts.
