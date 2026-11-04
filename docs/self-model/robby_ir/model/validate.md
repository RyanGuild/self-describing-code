# `robby_ir::model::validate`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn validate(doc: &Document) -> Vec<Diagnostic>` |
| location | [`crates/robby-ir/src/model.rs:450`](../../../../crates/robby-ir/src/model.rs#L450) |
| provenance | extractor · confidence 1.00 |

> Structural checks that do not depend on source text.
> 
> Returns diagnostics rather than a `Result`: a graph with warnings is still
> renderable, and hiding soft problems behind a hard error is how they rot.

```rust
/// Structural checks that do not depend on source text.
///
/// Returns diagnostics rather than a `Result`: a graph with warnings is still
/// renderable, and hiding soft problems behind a hard error is how they rot.
pub fn validate(doc: &Document) -> Vec<Diagnostic> {
    let mut out = Vec::new();

    if doc.schema_version != SCHEMA_VERSION {
        out.push(Diagnostic::error(
            "schema/version",
            format!(
                "document schema_version {} does not match tool {}",
                doc.schema_version, SCHEMA_VERSION
            ),
        ));
    }

    for (id, node) in &doc.nodes {
        if let Some(parent) = &node.parent {
            if !doc.nodes.contains_key(parent) {
                out.push(
                    Diagnostic::error(
                        "node/parent-missing",
                        format!("parent {parent} is not in the document"),
                    )
                    .about(id.clone()),
                );
            }
        }
        for child in &node.children {
            if !doc.nodes.contains_key(child) {
                out.push(
                    Diagnostic::error(
                        "node/child-missing",
                        format!("child {child} is not in the document"),
                    )
                    .about(id.clone()),
                );
            }
        }
        for edge in &node.edges {
            if edge.resolved && !doc.nodes.contains_key(&edge.to) {
                out.push(
                    Diagnostic::error(
                        "edge/target-missing",
                        format!("resolved edge points at missing node {}", edge.to),
                    )
                    .about(id.clone()),
                );
            }
            if !edge.resolved {
                out.push(
                    Diagnostic::warning(
                        "edge/unresolved",
                        format!("edge to {} was never resolved", edge.to),
                    )
                    .about(id.clone()),
                );
            }
        }
        if !crate::kind::is_known(&node.kind) {
            out.push(
                Diagnostic::warning(
                    "kind/unknown-namespace",
                    format!("kind `{}` has an unknown namespace", node.kind),
                )
                .about(id.clone()),
            );
        }
    }

    for path in &doc.code_paths {
        if !doc.nodes.contains_key(&path.entry) {
            out.push(
                Diagnostic::error(
                    "codepath/entry-missing",
                    format!("code path entry {} is not in the document", path.entry),
                )
                .about(path.entry.clone()),
            );
        }
        for edge in &path.edges {
            if !doc.nodes.contains_key(&edge.from) {
                out.push(
                    Diagnostic::error(
                        "codepath/from-missing",
                        format!("code path edge from missing node {}", edge.from),
                    )
                    .about(edge.from.clone()),
                );
            }
            if !doc.nodes.contains_key(&edge.to) {
                out.push(
                    Diagnostic::error(
                        "codepath/to-missing",
                        format!("code path edge to missing node {}", edge.to),
                    )
                    .about(edge.to.clone()),
                );
            }
        }
    }

    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant validate as validate()
    participant Vec__new as Vec::new
    participant Diagnostic__error as Diagnostic::error
    participant push as push
    participant contains_key as contains_key
    participant clone as clone
    participant about as about
    participant Diagnostic__warning as Diagnostic::warning
    participant crate__kind__is_known as crate::kind::is_known
    validate-->>Vec__new: Vec::new()
    alt doc.schema_version != SCHEMA_VERSION
    validate-->>validate: format!( 'document schema_version {} does not match tool {}', doc.sch...
    validate-->>Diagnostic__error: Diagnostic::error( 'schema/version', format!( 'document schema_versio...
    validate-->>push: out.push(Diagnostic::error( 'schema/version', format!( 'document sche...
    end
    loop &doc.nodes
    alt let Some(parent) = &node.parent
    validate-->>contains_key: doc.nodes.contains_key(parent)
    alt !doc.nodes.contains_key(parent)
    validate-->>validate: format!('parent {parent} is not in the document')
    validate-->>Diagnostic__error: Diagnostic::error( 'node/parent-missing', format!('parent {parent} is...
    validate-->>clone: id.clone()
    validate-->>about: Diagnostic::error( 'node/parent-missing', format!('parent {parent} is...
    validate-->>push: out.push( Diagnostic::error( 'node/parent-missing', format!('parent {...
    end
    end
    loop &node.children
    validate-->>contains_key: doc.nodes.contains_key(child)
    alt !doc.nodes.contains_key(child)
    validate-->>validate: format!('child {child} is not in the document')
    validate-->>Diagnostic__error: Diagnostic::error( 'node/child-missing', format!('child {child} is no...
    validate-->>clone: id.clone()
    validate-->>about: Diagnostic::error( 'node/child-missing', format!('child {child} is no...
    validate-->>push: out.push( Diagnostic::error( 'node/child-missing', format!('child {ch...
    end
    end
    loop &node.edges
    validate-->>contains_key: doc.nodes.contains_key(&edge.to)
    alt edge.resolved && !doc.nodes.contains_key(&edge.to)
    validate-->>validate: format!('resolved edge points at missing node {}', edge.to)
    validate-->>Diagnostic__error: Diagnostic::error( 'edge/target-missing', format!('resolved edge poin...
    validate-->>clone: id.clone()
    validate-->>about: Diagnostic::error( 'edge/target-missing', format!('resolved edge poin...
    validate-->>push: out.push( Diagnostic::error( 'edge/target-missing', format!('resolved...
    end
    alt !edge.resolved
    validate-->>validate: format!('edge to {} was never resolved', edge.to)
    validate-->>Diagnostic__warning: Diagnostic::warning( 'edge/unresolved', format!('edge to {} was never...
    validate-->>clone: id.clone()
    validate-->>about: Diagnostic::warning( 'edge/unresolved', format!('edge to {} was never...
    validate-->>push: out.push( Diagnostic::warning( 'edge/unresolved', format!('edge to {}...
    end
    end
    validate-->>crate__kind__is_known: crate::kind::is_known(&node.kind)
    alt !crate::kind::is_known(&node.kind)
    validate-->>validate: format!('kind `{}` has an unknown namespace', node.kind)
    validate-->>Diagnostic__warning: Diagnostic::warning( 'kind/unknown-namespace', format!('kind `{}` has...
    validate-->>clone: id.clone()
    validate-->>about: Diagnostic::warning( 'kind/unknown-namespace', format!('kind `{}` has...
    validate-->>push: out.push( Diagnostic::warning( 'kind/unknown-namespace', format!('kin...
    end
    end
    loop &doc.code_paths
    validate-->>contains_key: doc.nodes.contains_key(&path.entry)
    alt !doc.nodes.contains_key(&path.entry)
    validate-->>validate: format!('code path entry {} is not in the document', path.entry)
    validate-->>Diagnostic__error: Diagnostic::error( 'codepath/entry-missing', format!('code path entry...
    validate-->>clone: path.entry.clone()
    validate-->>about: Diagnostic::error( 'codepath/entry-missing', format!('code path entry...
    validate-->>push: out.push( Diagnostic::error( 'codepath/entry-missing', format!('code ...
    end
    loop &path.edges
    validate-->>contains_key: doc.nodes.contains_key(&edge.from)
    alt !doc.nodes.contains_key(&edge.from)
    validate-->>validate: format!('code path edge from missing node {}', edge.from)
    validate-->>Diagnostic__error: Diagnostic::error( 'codepath/from-missing', format!('code path edge f...
    validate-->>clone: edge.from.clone()
    validate-->>about: Diagnostic::error( 'codepath/from-missing', format!('code path edge f...
    validate-->>push: out.push( Diagnostic::error( 'codepath/from-missing', format!('code p...
    end
    validate-->>contains_key: doc.nodes.contains_key(&edge.to)
    alt !doc.nodes.contains_key(&edge.to)
    validate-->>validate: format!('code path edge to missing node {}', edge.to)
    validate-->>Diagnostic__error: Diagnostic::error( 'codepath/to-missing', format!('code path edge to ...
    validate-->>clone: edge.to.clone()
    validate-->>about: Diagnostic::error( 'codepath/to-missing', format!('code path edge to ...
    validate-->>push: out.push( Diagnostic::error( 'codepath/to-missing', format!('code pat...
    end
    end
    end
```

51 steps · 0 observed · 51 possible only

**Edges**

- calls → [`robby_ir::model::impl Edge::new`](index.md)
- calls → [`robby_ir::model::impl Diagnostic::error`](index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::model::impl Diagnostic::error`](index.md)
- calls → [`robby_ir::model::impl Diagnostic::about`](index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::model::impl Diagnostic::error`](index.md)
- calls → [`robby_ir::model::impl Diagnostic::about`](index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::model::impl Diagnostic::error`](index.md)
- calls → [`robby_ir::model::impl Diagnostic::about`](index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::model::impl Diagnostic::warning`](index.md)
- calls → [`robby_ir::model::impl Diagnostic::about`](index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::kind::is_known`](../kind/is-known.md)
- calls → [`robby_ir::model::impl Diagnostic::warning`](index.md)
- calls → [`robby_ir::model::impl Diagnostic::about`](index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::model::impl Diagnostic::error`](index.md)
- calls → [`robby_ir::model::impl Diagnostic::about`](index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::model::impl Diagnostic::error`](index.md)
- calls → [`robby_ir::model::impl Diagnostic::about`](index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::model::impl Diagnostic::error`](index.md)
- calls → [`robby_ir::model::impl Diagnostic::about`](index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)

## References

- calls → [`robby_ir::kind`](../kind/index.md) — is_known
- calls → [`robby_ir::model`](index.md) — about, error, new, warning
- calls → [`robby_render::report`](../../robby_render/report/index.md) — push

## Referenced by

- calls ← [`robby_render::pipeline::from_document`](../../robby_render/pipeline/from-document.md)

