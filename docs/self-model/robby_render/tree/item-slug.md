# `robby_render::tree::item_slug`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn item_slug(name: &str, id: &NodeId, used: &mut BTreeSet<String>) -> String` |
| location | [`crates/robby-render/src/tree.rs:827`](../../../../crates/robby-render/src/tree.rs#L827) |
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
    participant chars as chars
    participant is_ascii_alphanumeric as is_ascii_alphanumeric
    participant to_ascii_lowercase as to_ascii_lowercase
    participant push as push
    participant is_empty as is_empty
    participant ends_with as ends_with
    participant pop as pop
    participant push_str as push_str
    participant clone as clone
    participant insert as insert
    participant as_str as as_str
    participant crate__stable_id as crate::stable_id
    item_slug-->>String__new: String::new()
    item_slug-->>chars: name.chars()
    loop name.chars()
    item_slug-->>is_ascii_alphanumeric: ch.is_ascii_alphanumeric()
    alt ch.is_ascii_alphanumeric()
    item_slug-->>to_ascii_lowercase: ch.to_ascii_lowercase()
    item_slug-->>push: base.push(ch.to_ascii_lowercase())
    else else
    item_slug-->>is_empty: base.is_empty()
    item_slug-->>ends_with: base.ends_with('-')
    alt !base.is_empty() && !base.ends_with('-')
    item_slug-->>push: base.push('-')
    end
    end
    end
    item_slug-->>ends_with: base.ends_with('-')
    loop base.ends_with('-')
    item_slug-->>pop: base.pop()
    end
    item_slug-->>is_empty: base.is_empty()
    alt base.is_empty()
    item_slug-->>push_str: base.push_str('item')
    end
    item_slug-->>clone: base.clone()
    item_slug-->>insert: used.insert(base.clone())
    alt used.insert(base.clone())
    item_slug-->>item_slug: return
    end
    item_slug-->>as_str: id.as_str()
    item_slug-->>crate__stable_id: crate::stable_id(id.as_str())
    item_slug-->>item_slug: format!('{base}-{}', &suffix[..suffix.len().min(8)])
    item_slug-->>clone: alt.clone()
    item_slug-->>insert: used.insert(alt.clone())
```

20 steps · 0 observed · 20 possible only

**Edges**

- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::report::impl Report::push`](../report/index.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../view/links/index.md)
- calls → [`robby_render::report::impl Report::push`](../report/index.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../view/links/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_render::view::comments::impl NoteKind::as_str`](../view/comments/index.md)
- calls → [`robby_render::stable_id`](../stable-id.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)

## References

- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — insert
- calls → [`robby_render`](../index.md) — stable_id
- calls → [`robby_render::report`](../report/index.md) — new, push
- calls → [`robby_render::view::comments`](../view/comments/index.md) — as_str
- calls → [`robby_render::view::links`](../view/links/index.md) — is_empty

## Referenced by

- calls ← [`robby_render::tree::item_slugs`](item-slugs.md)

