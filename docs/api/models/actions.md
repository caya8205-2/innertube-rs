# Model: Actions (`src/models/actions.rs`)

```rust
use innertube_rs::models::actions::ActionResult;
```

---

## Structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub success: bool,
    pub status: String,
    pub error_message: Option<String>,
}
```
