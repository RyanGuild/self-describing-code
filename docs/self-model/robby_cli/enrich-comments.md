# `robby_cli::enrich_comments`

[Self-model](../README.md) · [robby_cli](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn enrich_comments(     ir: &std::path::Path,     source_root: &std::path::Path,     out: Option<&std::path::Path>,     decision_model: Option<String>,     note_model: Option<String>,     limit: Option<usize>, ) -> Result<ExitCode>` |
| location | [`crates/robby-cli/src/main.rs:278`](../../../crates/robby-cli/src/main.rs#L278) |
| provenance | extractor · confidence 1.00 |

```rust
fn enrich_comments(
    ir: &std::path::Path,
    source_root: &std::path::Path,
    out: Option<&std::path::Path>,
    decision_model: Option<String>,
    note_model: Option<String>,
    limit: Option<usize>,
) -> Result<ExitCode> {
    let mut loaded = pipeline::load(ir, source_root)?;
    let client = robby_agent::OpenRouterClient::from_env()?;
    let mut opts = robby_agent::EnrichOpts::default();
    if let Some(m) = decision_model {
        opts.decision_model = m;
    }
    if let Some(m) = note_model {
        opts.note_model = m;
    }
    opts.limit = limit;

    let stats = robby_agent::enrich_comments(&mut loaded.document, &loaded.sources, &client, &opts)?;
    let out_path = out.unwrap_or(ir);
    pipeline::write_document(&loaded.document, out_path)?;
    println!(
        "enriched {} comment(s): {} relevant, {} noise, {} noted → {}",
        stats.decided, stats.relevant, stats.noise, stats.noted, out_path.display()
    );
    Ok(ExitCode::SUCCESS)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant enrich_comments as enrich_comments()
    participant pipeline__load as pipeline::load
    participant robby_agent__OpenRouterClient__from_env as robby_agent::OpenRouterClient::from_env
    participant robby_agent__EnrichOpts__default as robby_agent::EnrichOpts::default
    participant robby_agent__enrich_comments as robby_agent::enrich_comments
    participant unwrap_or as unwrap_or
    participant pipeline__write_document as pipeline::write_document
    participant Ok as Ok
    enrich_comments-->>pipeline__load: pipeline::load(ir, source_root)
    enrich_comments-->>enrich_comments: ? (propagate)
    enrich_comments-->>robby_agent__OpenRouterClient__from_env: robby_agent::OpenRouterClient::from_env()
    enrich_comments-->>enrich_comments: ? (propagate)
    enrich_comments-->>robby_agent__EnrichOpts__default: robby_agent::EnrichOpts::default()
    enrich_comments-->>robby_agent__enrich_comments: robby_agent::enrich_comments(&mut loaded.document, &loaded.sources, &...
    enrich_comments-->>enrich_comments: ? (propagate)
    enrich_comments-->>unwrap_or: out.unwrap_or(ir)
    enrich_comments-->>pipeline__write_document: pipeline::write_document(&loaded.document, out_path)
    enrich_comments-->>enrich_comments: ? (propagate)
    enrich_comments-->>enrich_comments: println!( 'enriched {} comment(s): {} relevant, {} noise, {} noted → ...
    enrich_comments-->>Ok: Ok(ExitCode::SUCCESS)
```

12 steps · 0 observed · 12 possible only

**Edges**

- calls → [`robby_render::pipeline::load`](../robby_render/pipeline/load.md)
- calls → [`robby_agent::openrouter::impl OpenRouterClient::from_env`](../robby_agent/openrouter/index.md)
- calls → [`robby_ir::process::impl Default for Process::default`](../robby_ir/process/index.md)
- calls → [`robby_cli::enrich_comments`](enrich-comments.md)
- calls → [`robby_render::pipeline::write_document`](../robby_render/pipeline/write-document.md)

## References

- calls → [`robby_agent::openrouter`](../robby_agent/openrouter/index.md) — from_env
- calls → [`robby_ir::process`](../robby_ir/process/index.md) — default
- calls → [`robby_render::pipeline`](../robby_render/pipeline/index.md) — load, write_document

## Referenced by

- calls ← [`robby_cli::run`](run.md)

