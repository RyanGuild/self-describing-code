# `robby_render::pipeline::impl Loaded`

[Self-model](../../README.md) · [pipeline](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-render/src/pipeline.rs:25`](../../../../crates/robby-render/src/pipeline.rs#L25) |
| provenance | extractor · confidence 1.00 |

```rust
impl Loaded {
    /// True when the coverage gate passes and there are no error diagnostics.
    pub fn is_ok(&self) -> bool {
        self.coverage.is_complete()
            && !self
                .diagnostics
                .iter()
                .any(|d| d.severity == Severity::Error)
    }

    /// One-line summary for build output and CI logs.
    pub fn summary(&self) -> String {
        let files = self.coverage.files.len();
        let claimed: u32 = self.coverage.files.iter().map(|f| f.covered_bytes).sum();
        let unclaimed: u32 = self
            .coverage
            .files
            .iter()
            .map(robby_ir::FileCoverage::uncovered_bytes)
            .sum();
        format!(
            "{files} file(s), {claimed} bytes claimed, {unclaimed} unclaimed, {} diagnostic(s)",
            self.diagnostics.len()
        )
    }
}
```

## References

- calls → [`robby_ir::coverage`](../../robby_ir/coverage/index.md) — is_complete

## Referenced by

- calls ← [`robby_cli::build::main`](../../robby_cli/build/main.md)
- calls ← [`robby_cli::check`](../../robby_cli/check.md)
- calls ← [`robby_cli::example_self_model::run`](../../robby_cli/example_self_model/run.md)
- calls ← [`robby_cli::print_human`](../../robby_cli/print-human.md)

### `is_ok`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn is_ok(1)` |
| location | [`crates/robby-render/src/pipeline.rs:26`](../../../../crates/robby-render/src/pipeline.rs#L26) |
| provenance | extractor · confidence 1.00 |

> True when the coverage gate passes and there are no error diagnostics.

```rust
/// True when the coverage gate passes and there are no error diagnostics.
    pub fn is_ok(&self) -> bool {
        self.coverage.is_complete()
            && !self
                .diagnostics
                .iter()
                .any(|d| d.severity == Severity::Error)
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant is_ok as is_ok()
    participant is_complete as is_complete
    participant iter as iter
    participant any as any
    is_ok-->>is_complete: self.coverage.is_complete()
    is_ok-->>iter: self .diagnostics .iter()
    is_ok-->>any: self .diagnostics .iter() .any(|d| d.severity == Severity::Error)
```

3 steps · 0 observed · 3 possible only

**Edges**

- calls → [`robby_ir::coverage::impl FileCoverage::is_complete`](../../robby_ir/coverage/index.md)

### `summary`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn summary(1)` |
| location | [`crates/robby-render/src/pipeline.rs:35`](../../../../crates/robby-render/src/pipeline.rs#L35) |
| provenance | extractor · confidence 1.00 |

> One-line summary for build output and CI logs.

```rust
/// One-line summary for build output and CI logs.
    pub fn summary(&self) -> String {
        let files = self.coverage.files.len();
        let claimed: u32 = self.coverage.files.iter().map(|f| f.covered_bytes).sum();
        let unclaimed: u32 = self
            .coverage
            .files
            .iter()
            .map(robby_ir::FileCoverage::uncovered_bytes)
            .sum();
        format!(
            "{files} file(s), {claimed} bytes claimed, {unclaimed} unclaimed, {} diagnostic(s)",
            self.diagnostics.len()
        )
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant summary as summary()
    participant len as len
    participant iter as iter
    participant map as map
    participant sum as sum
    summary-->>len: self.coverage.files.len()
    summary-->>iter: self.coverage.files.iter()
    summary-->>map: self.coverage.files.iter().map(|f| f.covered_bytes)
    summary-->>sum: self.coverage.files.iter().map(|f| f.covered_bytes).sum()
    summary-->>iter: self .coverage .files .iter()
    summary-->>map: self .coverage .files .iter() .map(robby_ir::FileCoverage::uncovered_...
    summary-->>sum: self .coverage .files .iter() .map(robby_ir::FileCoverage::uncovered_...
    summary-->>summary: format!( '{files} file(s), {claimed} bytes claimed, {unclaimed} uncla...
```

8 steps · 0 observed · 8 possible only

