# `robby_ir::coverage::scan_trivia`

[Self-model](../../README.md) · [coverage](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn scan_trivia(bytes: &[u8], start: u32, end: u32) -> Result<TriviaFlags, u32>` |
| location | [`crates/robby-ir/src/coverage.rs:296`](../../../../crates/robby-ir/src/coverage.rs#L296) |
| provenance | extractor · confidence 1.00 |

> Walk a byte range, accepting only whitespace, comments and attributes.
> 
> Returns the byte offset of the first token that is none of those, which is
> what makes the gap report actionable: it points at the exact byte that is
> missing from the map.

```rust
/// Walk a byte range, accepting only whitespace, comments and attributes.
///
/// Returns the byte offset of the first token that is none of those, which is
/// what makes the gap report actionable: it points at the exact byte that is
/// missing from the map.
fn scan_trivia(bytes: &[u8], start: u32, end: u32) -> Result<TriviaFlags, u32> {
    let mut flags = TriviaFlags::default();
    let mut i = start;
    while i < end {
        let c = bytes[i as usize];
        if c.is_ascii_whitespace() {
            flags.whitespace = true;
            i += 1;
        } else if c == b'/' && i + 1 < end && bytes[(i + 1) as usize] == b'/' {
            flags.comment = true;
            while i < end && bytes[i as usize] != b'\n' {
                i += 1;
            }
        } else if c == b'/' && i + 1 < end && bytes[(i + 1) as usize] == b'*' {
            flags.comment = true;
            let mut depth = 1u32;
            i += 2;
            while i < end && depth > 0 {
                if i + 1 < end && bytes[i as usize] == b'/' && bytes[(i + 1) as usize] == b'*' {
                    depth += 1;
                    i += 2;
                } else if i + 1 < end && bytes[i as usize] == b'*' && bytes[(i + 1) as usize] == b'/' {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
        } else if c == b'#' {
            match skip_attribute(bytes, i, end) {
                Some(next) => {
                    flags.attribute = true;
                    i = next;
                }
                None => return Err(i),
            }
        } else {
            return Err(i);
        }
    }
    Ok(flags)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant scan_trivia as scan_trivia()
    participant TriviaFlags__default as TriviaFlags::default
    participant Err as Err
    participant Ok as Ok
    scan_trivia-->>TriviaFlags__default: TriviaFlags::default()
    scan_trivia-->>scan_trivia: start
    loop i < end
    scan_trivia-->>scan_trivia: bytes[i as usize]
    alt c.is_ascii_whitespace()
    scan_trivia-->>scan_trivia: flags.whitespace = true
    scan_trivia-->>scan_trivia: i += 1
    else else
    alt c == b'/' && i + 1 < end && bytes[(i + 1) as usize] == b'/'
    scan_trivia-->>scan_trivia: flags.comment = true
    loop i < end && bytes[i as usize] != b'\n'
    scan_trivia-->>scan_trivia: i += 1
    end
    alt c == b'/' && i + 1 < end && bytes[(i + 1) as usize] == b'*'
    scan_trivia-->>scan_trivia: flags.comment = true
    scan_trivia-->>scan_trivia: 1u32
    scan_trivia-->>scan_trivia: i += 2
    loop i < end && depth > 0
    alt i + 1 < end && bytes[i as usize] == b'/' && bytes[(i + 1) as usize] =...
    scan_trivia-->>scan_trivia: depth += 1
    scan_trivia-->>scan_trivia: i += 2
    else else
    alt i + 1 < end && bytes[i as usize] == b'*' && bytes[(i + 1) as usize] =...
    scan_trivia-->>scan_trivia: depth -= 1
    scan_trivia-->>scan_trivia: i += 2
    else else
    scan_trivia-->>scan_trivia: i += 1
    end
    end
    end
    alt c == b' '
    alt match skip_attribute(bytes, i, end)
    scan_trivia-->>scan_trivia: flags.attribute = true
    scan_trivia-->>scan_trivia: i = next
    else None
    scan_trivia-->>Err: Err(i)
    scan_trivia-->>scan_trivia: return
    end
    scan_trivia-->>Err: Err(i)
    scan_trivia-->>scan_trivia: return
    end
    end
    end
    end
    end
    scan_trivia-->>Ok: Ok(flags)
```

22 steps · 0 observed · 22 possible only

**Edges**

- calls → [`robby_ir::model::impl Default for Provenance::default`](../model/index.md)

