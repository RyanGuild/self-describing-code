# `robby_render::markdown::node_section`

[Self-model](../../README.md) · [markdown](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn node_section(     out: &mut String,     doc: &Document,     node: &Node,     sources: Option<&BTreeMap<String, String>>, )` |
| location | [`crates/robby-render/src/markdown.rs:102`](../../../../crates/robby-render/src/markdown.rs#L102) |
| provenance | extractor · confidence 1.00 |

```rust
fn node_section(
    out: &mut String,
    doc: &Document,
    node: &Node,
    sources: Option<&BTreeMap<String, String>>,
) {
    let depth = heading_depth(doc, node);
    let hashes = "#".repeat(depth.min(6));

    let _ = writeln!(out, "{hashes} `{}`\n", node.fqn);
    let _ = writeln!(out, "| | |");
    let _ = writeln!(out, "|---|---|");
    let _ = writeln!(out, "| kind | `{}` |", node.kind);
    let _ = writeln!(out, "| id | `{}` |", node.id);
    if let Some(sig) = &node.signature {
        let _ = writeln!(out, "| signature | `{}` |", cell(sig));
    }
    if let Some(span) = &node.span {
        let _ = writeln!(out, "| location | `{}` |", span.location());
        let _ = writeln!(
            out,
            "| span | bytes {}..{} |",
            span.start.byte, span.end.byte
        );
    }
    let _ = writeln!(
        out,
        "| provenance | {} · confidence {:.2} |",
        source_name(node.provenance.source),
        node.provenance.confidence
    );
    if !node.tags.is_empty() {
        let _ = writeln!(out, "| tags | {} |", node.tags.join(", "));
    }
    out.push('\n');

    if let Some(docs) = &node.docs {
        for line in docs.lines() {
            let _ = writeln!(out, "> {line}");
        }
        out.push('\n');
    }

    if let Some(span) = &node.span {
        if let Some(source) = sources.and_then(|s| s.get(&span.file)) {
            if let Some(slice) = source.get(span.start.byte as usize..span.end.byte as usize) {
                out.push_str("```rust\n");
                out.push_str(slice.trim_end());
                out.push_str("\n```\n\n");
            }
        }
    }

    if let Some(process) = &node.process {
        out.push_str("```mermaid\n");
        out.push_str(&sequence(process));
        out.push_str("```\n\n");

        let observed = process.steps.iter().filter(|s| s.observed).count();
        let _ = writeln!(
            out,
            "{} steps · {} observed · {} possible only\n",
            process.steps.len(),
            observed,
            process.steps.len() - observed
        );
    }

    if !node.edges.is_empty() {
        let _ = writeln!(out, "**Edges**\n");
        for edge in &node.edges {
            let marker = if edge.resolved { "" } else { " (unresolved)" };
            let _ = writeln!(
                out,
                "- {} → `{}`{marker}",
                edge_name(edge.kind),
                edge.to
            );
        }
        out.push('\n');
    }

    if !node.children.is_empty() {
        let names: Vec<String> = node
            .children
            .iter()
            .filter_map(|c| doc.get(c))
            .map(|c| format!("`{}`", c.fqn))
            .collect();
        let _ = writeln!(out, "**Children** — {}\n", names.join(", "));
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant node_section as node_section()
    participant heading_depth as heading_depth
    participant repeat as repeat
    participant push as push
    participant push_str as push_str
    participant count as count
    participant collect as collect
    node_section-->>heading_depth: heading_depth(doc, node)
    node_section-->>repeat: ' '.repeat(depth.min(6))
    node_section-->>node_section: writeln!(out, '{hashes} `{}`\n', node.fqn)
    node_section-->>node_section: writeln!(out, '| | |')
    node_section-->>node_section: writeln!(out, '|---|---|')
    node_section-->>node_section: writeln!(out, '| kind | `{}` |', node.kind)
    node_section-->>node_section: writeln!(out, '| id | `{}` |', node.id)
    alt let Some(sig) = &node.signature
    node_section-->>node_section: writeln!(out, '| signature | `{}` |', cell(sig))
    end
    alt let Some(span) = &node.span
    node_section-->>node_section: writeln!(out, '| location | `{}` |', span.location())
    node_section-->>node_section: writeln!( out, '| span | bytes {}..{} |', span.start.byte, span.end.b...
    end
    node_section-->>node_section: writeln!( out, '| provenance | {} · confidence {:.2} |', source_name(...
    alt !node.tags.is_empty()
    node_section-->>node_section: writeln!(out, '| tags | {} |', node.tags.join(', '))
    end
    node_section-->>push: out.push('\n')
    alt let Some(docs) = &node.docs
    loop docs.lines()
    node_section-->>node_section: writeln!(out, '> {line}')
    end
    node_section-->>push: out.push('\n')
    end
    alt let Some(span) = &node.span
    alt let Some(source) = sources.and_then(|s| s.get(&span.file))
    alt let Some(slice) = source.get(span.start.byte as usize..span.end.byte ...
    node_section-->>push_str: out.push_str('```rust\n')
    node_section-->>push_str: out.push_str(slice.trim_end())
    node_section-->>push_str: out.push_str('\n```\n\n')
    end
    end
    end
    alt let Some(process) = &node.process
    node_section-->>push_str: out.push_str('```mermaid\n')
    node_section-->>push_str: out.push_str(&sequence(process))
    node_section-->>push_str: out.push_str('```\n\n')
    node_section-->>count: process.steps.iter().filter(|s| s.observed).count()
    node_section-->>node_section: writeln!( out, '{} steps · {} observed · {} possible only\n', process...
    end
    alt !node.edges.is_empty()
    node_section-->>node_section: writeln!(out, '**Edges**\n')
    loop &node.edges
    alt edge.resolved
    node_section-->>node_section: ''
    else else
    node_section-->>node_section: ' (unresolved)'
    end
    node_section-->>node_section: writeln!( out, '- {} → `{}`{marker}', edge_name(edge.kind), edge.to )
    end
    node_section-->>push: out.push('\n')
    end
    alt !node.children.is_empty()
    node_section-->>collect: node .children .iter() .filter_map(|c| doc.get(c)) .map(|c| format!('...
    node_section-->>node_section: writeln!(out, '**Children** — {}\n', names.join(', '))
    end
```

30 steps · 0 observed · 30 possible only

**Edges**

- calls → [`robby_render::markdown::heading_depth`](heading-depth.md)

