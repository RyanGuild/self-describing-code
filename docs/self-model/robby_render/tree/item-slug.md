# `robby_render::tree::item_slug`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn item_slug(name: &str, id: &NodeId, used: &mut BTreeSet<String>) -> String` |
| location | [`crates/robby-render/src/tree.rs:648`](../../../../crates/robby-render/src/tree.rs#L648) |
| provenance | extractor · confidence 1.00 |

> A filesystem-safe, human-readable page name, disambiguated on collision.

```rust
/// A filesystem-safe, human-readable page name, disambiguated on collision.
fn item_slug(name: &str, id: &NodeId, used: &mut BTreeSet<String>) -> String {
    let mut base = String::new();
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            base.push(ch.to_ascii_lowercase());
        } else if !base.is_empty() && !base.ends_with('-') {
            base.push('-');
        }
    }
    while base.ends_with('-') {
        base.pop();
    }
    if base.is_empty() {
        base.push_str("item");
    }

    if used.insert(base.clone()) {
        return base;
    }

    let suffix = crate::stable_id(id.as_str());
    let alt = format!("{base}-{}", &suffix[..suffix.len().min(8)]);
    used.insert(alt.clone());
    alt
}
```

```mermaid
sequenceDiagram
    autonumber
    participant item_slug as item_slug()
    participant String__new as String::new
    participant push as push
    participant pop as pop
    participant push_str as push_str
    participant crate__stable_id as crate::stable_id
    participant insert as insert
    item_slug-->>String__new: String::new()
    loop name.chars()
    alt ch.is_ascii_alphanumeric()
    item_slug-->>push: base.push(ch.to_ascii_lowercase())
    else else
    alt !base.is_empty() && !base.ends_with('-')
    item_slug-->>push: base.push('-')
    end
    end
    end
    loop base.ends_with('-')
    item_slug-->>pop: base.pop()
    end
    alt base.is_empty()
    item_slug-->>push_str: base.push_str('item')
    end
    alt used.insert(base.clone())
    item_slug-->>item_slug: base
    item_slug-->>item_slug: return
    end
    item_slug-->>crate__stable_id: crate::stable_id(id.as_str())
    item_slug-->>item_slug: format!('{base}-{}', &suffix[..suffix.len().min(8)])
    item_slug-->>insert: used.insert(alt.clone())
    item_slug-->>item_slug: alt
```

11 steps · 0 observed · 11 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)
- calls → [`robby_render::stable_id`](../stable-id.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)

