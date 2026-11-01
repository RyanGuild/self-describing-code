# `robby_cli::print_human`

[Self-model](../README.md) · [robby_cli](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn print_human(loaded: &pipeline::Loaded)` |
| location | [`crates/robby-cli/src/main.rs:141`](../../../crates/robby-cli/src/main.rs#L141) |
| provenance | extractor · confidence 1.00 |

```rust
fn print_human(loaded: &pipeline::Loaded) {
    for diagnostic in &loaded.diagnostics {
        let tag = match diagnostic.severity {
            Severity::Error => "error",
            Severity::Warning => "warn ",
            Severity::Info => "info ",
        };
        match &diagnostic.node {
            Some(node) => println!("{tag} [{}] {} ({})", diagnostic.code, diagnostic.message, node),
            None => println!("{tag} [{}] {}", diagnostic.code, diagnostic.message),
        }
    }

    println!();
    println!("coverage");
    for file in loaded.coverage.worst_first() {
        println!(
            "  code {:>6.1}%  {:>6}/{:<6} claimed  {:>5} trivia  {}",
            file.code_ratio() * 100.0,
            file.covered_bytes,
            file.covered_bytes + file.uncovered_bytes(),
            file.trivia_bytes(),
            file.file
        );
        for gap in file.gaps.iter().filter(|g| g.reason.is_failure()) {
            println!(
                "           missing at {}  {:?}",
                gap.span.location(),
                gap.preview
            );
        }
    }

    println!();
    if loaded.is_ok() {
        println!("ok: every byte is owned or classified");
    } else {
        println!("FAIL: unclaimed code is invisible to the map");
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant print_human as print_human()
    loop &loaded.diagnostics
    alt match diagnostic.severity
    print_human-->>print_human: 'error'
    else Severity::Warning
    print_human-->>print_human: 'warn '
    else Severity::Info
    print_human-->>print_human: 'info '
    end
    alt match &diagnostic.node
    print_human-->>print_human: println!('{tag} [{}] {} ({})', diagnostic.code, diagnostic.message, n...
    else None
    print_human-->>print_human: println!('{tag} [{}] {}', diagnostic.code, diagnostic.message)
    end
    end
    print_human-->>print_human: println!()
    print_human-->>print_human: println!('coverage')
    loop loaded.coverage.worst_first()
    print_human-->>print_human: println!( ' code {:>6.1}% {:>6}/{:<6} claimed {:>5} trivia {}', file....
    loop file.gaps.iter().filter(|g| g.reason.is_failure())
    print_human-->>print_human: println!( ' missing at {} {:?}', gap.span.location(), gap.preview )
    end
    end
    print_human-->>print_human: println!()
    alt loaded.is_ok()
    print_human-->>print_human: println!('ok: every byte is owned or classified')
    else else
    print_human-->>print_human: println!('FAIL: unclaimed code is invisible to the map')
    end
```

12 steps · 0 observed · 12 possible only

