# `robby_render::markdown::coverage_section`

[Self-model](../../README.md) · [markdown](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn coverage_section(out: &mut String, report: &CoverageReport)` |
| location | [`crates/robby-render/src/markdown.rs:103`](../../../../crates/robby-render/src/markdown.rs#L103) |
| provenance | extractor · confidence 1.00 |

```rust
pub(crate) fn coverage_section(out: &mut String, report: &CoverageReport) {
    out.push_str("## Coverage\n\n");
    out.push_str("| File | Code | Claimed | Trivia | Status |\n");
    out.push_str("|---|---:|---:|---:|---|\n");

    for file in report.worst_first() {
        let status = if file.is_complete() {
            "ok".to_owned()
        } else {
            format!("**{} bytes unclaimed**", file.uncovered_bytes())
        };
        let _ = writeln!(
            out,
            "| `{}` | {:.1}% | {}/{} | {} | {} |",
            file.file,
            file.code_ratio() * 100.0,
            file.covered_bytes,
            file.covered_bytes + file.uncovered_bytes(),
            file.trivia_bytes(),
            status
        );
    }
    out.push('\n');

    let failures: Vec<_> = report.failures().collect();
    if failures.is_empty() {
        out.push_str("Every byte is owned by a node or classified as trivia.\n\n");
    } else {
        out.push_str("### Unclaimed code\n\n");
        for (file, gap) in failures {
            let _ = writeln!(
                out,
                "- `{}` — `{}` — `{}`",
                file.file,
                gap.span.location(),
                gap.preview
            );
        }
        out.push('\n');
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant coverage_section as coverage_section()
    participant push_str as push_str
    participant worst_first as worst_first
    participant is_complete as is_complete
    participant to_owned as to_owned
    participant push as push
    participant failures as failures
    participant collect as collect
    participant is_empty as is_empty
    coverage_section-->>push_str: out.push_str(' Coverage\n\n')
    coverage_section-->>push_str: out.push_str('| File | Code | Claimed | Trivia | Status |\n')
    coverage_section-->>push_str: out.push_str('|---|---:|---:|---:|---|\n')
    coverage_section-->>worst_first: report.worst_first()
    loop report.worst_first()
    coverage_section-->>is_complete: file.is_complete()
    alt file.is_complete()
    coverage_section-->>to_owned: 'ok'.to_owned()
    else else
    coverage_section-->>coverage_section: format!('**{} bytes unclaimed**', file.uncovered_bytes())
    end
    coverage_section-->>coverage_section: writeln!( out, '| `{}` | {:.1}% | {}/{} | {} | {} |', file.file, file...
    end
    coverage_section-->>push: out.push('\n')
    coverage_section-->>failures: report.failures()
    coverage_section-->>collect: report.failures().collect()
    coverage_section-->>is_empty: failures.is_empty()
    alt failures.is_empty()
    coverage_section-->>push_str: out.push_str('Every byte is owned by a node or classified as trivia.\...
    else else
    coverage_section-->>push_str: out.push_str(' Unclaimed code\n\n')
    loop failures
    coverage_section-->>coverage_section: writeln!( out, '- `{}` — `{}` — `{}`', file.file, gap.span.location()...
    end
    coverage_section-->>push: out.push('\n')
    end
```

16 steps · 0 observed · 16 possible only

**Edges**

- calls → [`robby_ir::coverage::impl CoverageReport::worst_first`](../../robby_ir/coverage/index.md)
- calls → [`robby_ir::coverage::impl FileCoverage::is_complete`](../../robby_ir/coverage/index.md)
- calls → [`robby_render::report::impl Report::push`](../report/index.md)
- calls → [`robby_ir::coverage::impl CoverageReport::failures`](../../robby_ir/coverage/index.md)
- calls → [`robby_render::view::select::collect`](../view/select/collect.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../view/links/index.md)
- calls → [`robby_render::report::impl Report::push`](../report/index.md)

## References

- calls → [`robby_ir::coverage`](../../robby_ir/coverage/index.md) — failures, is_complete, worst_first
- calls → [`robby_render::report`](../report/index.md) — push
- calls → [`robby_render::view::links`](../view/links/index.md) — is_empty
- calls → [`robby_render::view::select`](../view/select/index.md) — collect

## Referenced by

- calls ← [`robby_render::markdown::report`](report.md)
- calls ← [`robby_render::tree::global_index`](../tree/global-index.md)

