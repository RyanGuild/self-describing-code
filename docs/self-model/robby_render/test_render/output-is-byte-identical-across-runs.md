# `robby_render::test_render::output_is_byte_identical_across_runs`

[Self-model](../../README.md) · [test_render](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn output_is_byte_identical_across_runs()` |
| location | [`crates/robby-render/tests/render.rs:118`](../../../../crates/robby-render/tests/render.rs#L118) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn output_is_byte_identical_across_runs() {
    let p = process(vec![
        step("caller", "io", "a", true, vec![]),
        step("caller", "io", "b", false, vec![]),
    ]);
    assert_eq!(sequence(&p), sequence(&p));
}
```

```mermaid
sequenceDiagram
    autonumber
    participant output_is_byte_identical_across_runs as output_is_byte_identical_across_runs()
    participant process as process
    output_is_byte_identical_across_runs-->>process: process(vec![ step('caller', 'io', 'a', true, vec![]), step('caller',...
    output_is_byte_identical_across_runs-->>output_is_byte_identical_across_runs: assert_eq!(sequence(&p), sequence(&p))
```

2 steps · 0 observed · 2 possible only

**Edges**

- calls → [`robby_render::test_render::process`](process.md)

