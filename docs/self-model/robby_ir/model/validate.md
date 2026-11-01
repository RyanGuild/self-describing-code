# `robby_ir::model::validate`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn validate(doc: &Document) -> Vec<Diagnostic>` |
| location | [`crates/robby-ir/src/model.rs:414`](../../../../crates/robby-ir/src/model.rs#L414) |
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

    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant validate as validate()
    participant Vec__new as Vec::new
    participant push as push
    validate-->>Vec__new: Vec::new()
    alt doc.schema_version != SCHEMA_VERSION
    validate-->>push: out.push(Diagnostic::error( 'schema/version', format!( 'document sche...
    end
    loop &doc.nodes
    alt let Some(parent) = &node.parent
    alt !doc.nodes.contains_key(parent)
    validate-->>push: out.push( Diagnostic::error( 'node/parent-missing', format!('parent {...
    end
    end
    loop &node.children
    alt !doc.nodes.contains_key(child)
    validate-->>push: out.push( Diagnostic::error( 'node/child-missing', format!('child {ch...
    end
    end
    loop &node.edges
    alt edge.resolved && !doc.nodes.contains_key(&edge.to)
    validate-->>push: out.push( Diagnostic::error( 'edge/target-missing', format!('resolved...
    end
    alt !edge.resolved
    validate-->>push: out.push( Diagnostic::warning( 'edge/unresolved', format!('edge to {}...
    end
    end
    alt !crate::kind::is_known(&node.kind)
    validate-->>push: out.push( Diagnostic::warning( 'kind/unknown-namespace', format!('kin...
    end
    end
    validate-->>validate: out
```

8 steps · 0 observed · 8 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)

