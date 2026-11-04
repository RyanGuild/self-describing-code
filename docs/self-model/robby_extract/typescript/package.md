# `robby_extract::typescript::Package`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-extract/src/typescript.rs:55`](../../../../crates/robby-extract/src/typescript.rs#L55) |
| provenance | extractor · confidence 1.00 |

```rust
struct Package {
    name: String,
    dir: PathBuf,
    /// Paths relative to the extract root (workspace / fixture root).
    root: PathBuf,
    files: Vec<PathBuf>,
}
```

**Fields and variants**

- `name` — `crates/robby-extract/src/typescript.rs:56:5`
- `dir` — `crates/robby-extract/src/typescript.rs:57:5`
- `root` — `crates/robby-extract/src/typescript.rs:58:5`
- `files` — `crates/robby-extract/src/typescript.rs:60:5`

