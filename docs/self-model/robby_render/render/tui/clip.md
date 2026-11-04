# `robby_render::render::tui::clip`

[Self-model](../../../README.md) · [render::tui](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn clip(text: &str, max: usize) -> String` |
| location | [`crates/robby-render/src/render/tui.rs:586`](../../../../../crates/robby-render/src/render/tui.rs#L586) |
| provenance | extractor · confidence 1.00 |

```rust
fn clip(text: &str, max: usize) -> String {
    if UnicodeWidthStr::width(text) <= max {
        return text.to_owned();
    }
    if max <= 1 {
        return "…".to_owned();
    }
    let mut out = String::new();
    let mut width = 0;
    for ch in text.chars() {
        let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
        if width + ch_width + 1 > max {
            break;
        }
        out.push(ch);
        width += ch_width;
    }
    out.push('…');
    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant clip as clip()
    participant UnicodeWidthStr__width as UnicodeWidthStr::width
    participant to_owned as to_owned
    participant String__new as String::new
    participant chars as chars
    participant unicode_width__UnicodeWidthChar__width as unicode_width::UnicodeWidthChar::width
    participant unwrap_or as unwrap_or
    participant push as push
    clip-->>UnicodeWidthStr__width: UnicodeWidthStr::width(text)
    alt UnicodeWidthStr::width(text) <= max
    clip-->>to_owned: text.to_owned()
    clip-->>clip: return
    end
    alt max <= 1
    clip-->>to_owned: '…'.to_owned()
    clip-->>clip: return
    end
    clip-->>String__new: String::new()
    clip-->>chars: text.chars()
    loop text.chars()
    clip-->>unicode_width__UnicodeWidthChar__width: unicode_width::UnicodeWidthChar::width(ch)
    clip-->>unwrap_or: unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0)
    clip-->>push: out.push(ch)
    end
    clip-->>push: out.push('…')
```

11 steps · 0 observed · 11 possible only

**Edges**

- calls → [`robby_render::render::tui::impl TuiRenderer::new`](index.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)

## References

- calls → [`robby_render::render::tui`](index.md) — new
- calls → [`robby_render::report`](../../report/index.md) — push

## Referenced by

- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::comments`](index.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::process`](index.md)

