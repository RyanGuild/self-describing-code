# `robby_render::render::tui::ShowOptions`

[Self-model](../../../README.md) · [render::tui](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/render/tui.rs:18`](../../../../../crates/robby-render/src/render/tui.rs#L18) |
| provenance | extractor · confidence 1.00 |

> Options for one `robby show` invocation.

```rust
/// Options for one `robby show` invocation.
#[derive(Debug, Clone)]
pub struct ShowOptions {
    pub depth: usize,
    pub width: usize,
    pub all: bool,
    pub tree: bool,
    pub graph: bool,
    pub process: bool,
    pub source: bool,
    pub entry: bool,
    pub types: bool,
    pub dependencies: bool,
    pub externals: bool,
    pub comments: bool,
    pub references: bool,
}
```

**Fields and variants**

- `depth` — `crates/robby-render/src/render/tui.rs:21:5`
- `width` — `crates/robby-render/src/render/tui.rs:22:5`
- `all` — `crates/robby-render/src/render/tui.rs:23:5`
- `tree` — `crates/robby-render/src/render/tui.rs:24:5`
- `graph` — `crates/robby-render/src/render/tui.rs:25:5`
- `process` — `crates/robby-render/src/render/tui.rs:26:5`
- `source` — `crates/robby-render/src/render/tui.rs:27:5`
- `entry` — `crates/robby-render/src/render/tui.rs:28:5`
- `types` — `crates/robby-render/src/render/tui.rs:29:5`
- `dependencies` — `crates/robby-render/src/render/tui.rs:30:5`
- `externals` — `crates/robby-render/src/render/tui.rs:31:5`
- `comments` — `crates/robby-render/src/render/tui.rs:32:5`
- `references` — `crates/robby-render/src/render/tui.rs:33:5`

