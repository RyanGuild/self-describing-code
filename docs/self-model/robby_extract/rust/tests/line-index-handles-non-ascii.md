# `robby_extract::rust::line_index_handles_non_ascii`

[Self-model](../../../README.md) · [rust::tests](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn line_index_handles_non_ascii()` |
| location | [`crates/robby-extract/src/rust.rs:982`](../../../../../crates/robby-extract/src/rust.rs#L982) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
    fn line_index_handles_non_ascii() {
        let source = "let é = 1;\nfn a() {}";
        let index = LineIndex::new(source);

        // Start of line 2.
        let line_two = index.byte(LineColumn { line: 2, column: 0 });
        assert_eq!(line_two, source.find("fn a").unwrap() as u32);

        // Column 5 is just past the two-byte `é`, so the byte offset is 6.
        let after_e_acute = index.byte(LineColumn { line: 1, column: 5 });
        assert_eq!(after_e_acute, 6);
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant line_index_handles_non_ascii as line_index_handles_non_ascii()
    participant LineIndex__new as LineIndex::new
    participant byte as byte
    line_index_handles_non_ascii-->>LineIndex__new: LineIndex::new(source)
    line_index_handles_non_ascii-->>byte: index.byte(LineColumn { line: 2, column: 0 })
    line_index_handles_non_ascii-->>line_index_handles_non_ascii: assert_eq!(line_two, source.find('fn a').unwrap() as u32)
    line_index_handles_non_ascii-->>byte: index.byte(LineColumn { line: 1, column: 5 })
    line_index_handles_non_ascii-->>line_index_handles_non_ascii: assert_eq!(after_e_acute, 6)
```

5 steps · 0 observed · 5 possible only

**Edges**

- calls → [`robby_extract::rust::impl LineIndex::new`](../index.md)
- calls → [`robby_extract::rust::impl LineIndex::byte`](../index.md)
- calls → [`robby_extract::rust::impl LineIndex::byte`](../index.md)

## References

- calls → [`robby_extract::rust`](../index.md) — byte, new

