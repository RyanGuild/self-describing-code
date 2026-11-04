# `robby_cli::web`

[Self-model](../README.md) · [robby_cli](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn web(     ir: &std::path::Path,     source_root: &std::path::Path,     out: &std::path::Path,     watch: bool,     watch_interval_ms: u64, ) -> Result<ExitCode>` |
| location | [`crates/robby-cli/src/main.rs:230`](../../../crates/robby-cli/src/main.rs#L230) |
| provenance | extractor · confidence 1.00 |

```rust
fn web(
    ir: &std::path::Path,
    source_root: &std::path::Path,
    out: &std::path::Path,
    watch: bool,
    watch_interval_ms: u64,
) -> Result<ExitCode> {
    let count = write_web(ir, source_root, out)?;
    println!(
        "wrote {count} frame(s) to {}\nserve with: cd apps/site && npm install && npm run dev",
        out.display()
    );
    if !watch {
        return Ok(ExitCode::SUCCESS);
    }

    let mut last_mtime = ir_mtime(ir);
    println!(
        "watching {} every {watch_interval_ms}ms (Ctrl-C to stop)",
        ir.display()
    );
    loop {
        std::thread::sleep(std::time::Duration::from_millis(watch_interval_ms));
        let mtime = ir_mtime(ir);
        if mtime == last_mtime {
            continue;
        }
        last_mtime = mtime;
        match write_web(ir, source_root, out) {
            Ok(count) => println!("rebuilt {count} frame(s) → {}", out.display()),
            Err(err) => eprintln!("watch rebuild failed: {err:#}"),
        }
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant web as web()
    participant write_web as write_web
    participant Ok as Ok
    participant ir_mtime as ir_mtime
    participant std__time__Duration__from_millis as std::time::Duration::from_millis
    participant std__thread__sleep as std::thread::sleep
    web-->>write_web: write_web(ir, source_root, out)
    web-->>web: ? (propagate)
    web-->>web: println!( 'wrote {count} frame(s) to {}\nserve with: cd apps/site && ...
    alt !watch
    web-->>Ok: Ok(ExitCode::SUCCESS)
    web-->>web: return
    end
    web-->>ir_mtime: ir_mtime(ir)
    web-->>web: println!( 'watching {} every {watch_interval_ms}ms (Ctrl-C to stop)',...
    loop loop
    web-->>std__time__Duration__from_millis: std::time::Duration::from_millis(watch_interval_ms)
    web-->>std__thread__sleep: std::thread::sleep(std::time::Duration::from_millis(watch_interval_ms))
    web-->>ir_mtime: ir_mtime(ir)
    web-->>write_web: write_web(ir, source_root, out)
    alt match write_web(ir, source_root, out)
    web-->>web: println!('rebuilt {count} frame(s) → {}', out.display())
    else Err(err)
    web-->>web: eprintln!('watch rebuild failed: {err: }')
    end
    end
```

13 steps · 0 observed · 13 possible only

**Edges**

- calls → [`robby_cli::write_web`](write-web.md)
- calls → [`robby_cli::ir_mtime`](ir-mtime.md)
- calls → [`robby_cli::ir_mtime`](ir-mtime.md)
- calls → [`robby_cli::write_web`](write-web.md)

## References

- calls → [`robby_cli`](index.md) — ir_mtime, write_web

## Referenced by

- calls ← [`robby_cli::run`](run.md)

