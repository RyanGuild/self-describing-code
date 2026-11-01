# `robby_ir::coverage::skip_attribute`

[Self-model](../../README.md) · [coverage](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn skip_attribute(bytes: &[u8], start: u32, end: u32) -> Option<u32>` |
| location | [`crates/robby-ir/src/coverage.rs:344`](../../../../crates/robby-ir/src/coverage.rs#L344) |
| provenance | extractor · confidence 1.00 |

> Skip `#[...]` or `#![...]`, honouring nested brackets and string literals.

```rust
/// Skip `#[...]` or `#![...]`, honouring nested brackets and string literals.
fn skip_attribute(bytes: &[u8], start: u32, end: u32) -> Option<u32> {
    let mut i = start + 1;
    if i < end && bytes[i as usize] == b'!' {
        i += 1;
    }
    if i >= end || bytes[i as usize] != b'[' {
        return None;
    }
    let mut depth = 0i32;
    let mut in_string = false;
    while i < end {
        let c = bytes[i as usize];
        if in_string {
            if c == b'\\' {
                i += 2;
                continue;
            }
            if c == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        match c {
            b'"' => {
                in_string = true;
                i += 1;
            }
            b'[' => {
                depth += 1;
                i += 1;
            }
            b']' => {
                depth -= 1;
                i += 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => i += 1,
        }
    }
    None
}
```

```mermaid
sequenceDiagram
    autonumber
    participant skip_attribute as skip_attribute()
    participant Some as Some
    skip_attribute-->>skip_attribute: start + 1
    alt i < end && bytes[i as usize] == b'!'
    skip_attribute-->>skip_attribute: i += 1
    end
    alt i >= end || bytes[i as usize] != b'['
    skip_attribute-->>skip_attribute: None
    skip_attribute-->>skip_attribute: return
    end
    skip_attribute-->>skip_attribute: 0i32
    skip_attribute-->>skip_attribute: false
    loop i < end
    skip_attribute-->>skip_attribute: bytes[i as usize]
    alt in_string
    alt c == b'\\'
    skip_attribute-->>skip_attribute: i += 2
    skip_attribute-->>skip_attribute: continue
    end
    alt c == b'''
    skip_attribute-->>skip_attribute: in_string = false
    end
    skip_attribute-->>skip_attribute: i += 1
    skip_attribute-->>skip_attribute: continue
    end
    alt match c
    skip_attribute-->>skip_attribute: in_string = true
    skip_attribute-->>skip_attribute: i += 1
    else b'['
    skip_attribute-->>skip_attribute: depth += 1
    skip_attribute-->>skip_attribute: i += 1
    else b']'
    skip_attribute-->>skip_attribute: depth -= 1
    skip_attribute-->>skip_attribute: i += 1
    alt depth == 0
    skip_attribute-->>Some: Some(i)
    skip_attribute-->>skip_attribute: return
    end
    skip_attribute-->>skip_attribute: i += 1
    end
    end
    skip_attribute-->>skip_attribute: None
```

22 steps · 0 observed · 22 possible only

