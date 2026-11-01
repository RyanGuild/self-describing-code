# `robby_render::tree::render_node`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn render_node(     out: &mut String,     doc: &Document,     node: &Node,     depth: usize,     page: &Path,     sources: Option<&BTreeMap<String, String>>,     out_rel: &str,     emit_heading: bool, )` |
| location | [`crates/robby-render/src/tree.rs:389`](../../../../crates/robby-render/src/tree.rs#L389) |
| provenance | extractor · confidence 1.00 |

```rust
#[allow(clippy::too_many_arguments)]
fn render_node(
    out: &mut String,
    doc: &Document,
    node: &Node,
    depth: usize,
    page: &Path,
    sources: Option<&BTreeMap<String, String>>,
    out_rel: &str,
    emit_heading: bool,
) {
    if emit_heading {
        let hashes = "#".repeat(depth.min(6));
        let _ = writeln!(out, "{hashes} `{}`\n", node.name);
    }

    let _ = writeln!(out, "| | |");
    let _ = writeln!(out, "|---|---|");
    let _ = writeln!(out, "| kind | `{}` |", node.kind);
    if let Some(sig) = &node.signature {
        let _ = writeln!(out, "| signature | `{}` |", cell(sig));
    }
    if let Some(span) = &node.span {
        let _ = writeln!(out, "| location | {} |", source_link(page, out_rel, span));
    }
    let _ = writeln!(
        out,
        "| provenance | {} · confidence {:.2} |",
        source_name(node.provenance.source),
        node.provenance.confidence
    );
    out.push('\n');

    if let Some(docs) = &node.docs {
        for line in docs.lines() {
            let _ = writeln!(out, "> {line}");
        }
        out.push('\n');
    }

    if let (Some(span), Some(sources)) = (&node.span, sources) {
        if let Some(text) = slice(sources, span) {
            out.push_str("```rust\n");
            out.push_str(text.trim_end());
            out.push_str("\n```\n\n");
        }
    }

    if let Some(process) = &node.process {
        out.push_str("```mermaid\n");
        out.push_str(&sequence(process));
        out.push_str("```\n\n");

        let observed = process.steps.iter().filter(|step| step.observed).count();
        let _ = writeln!(
            out,
            "{} steps · {} observed · {} possible only\n",
            process.steps.len(),
            observed,
            process.steps.len() - observed
        );
    }

    if !node.edges.is_empty() {
        out.push_str("**Edges**\n\n");
        for edge in &node.edges {
            let marker = if edge.resolved { "" } else { " (unresolved)" };
            let target = doc.get(&edge.to);
            match target.and_then(|t| page_of_node(doc, t)) {
                Some(target_page) => {
                    let _ = writeln!(
                        out,
                        "- {} → [`{}`]({}){marker}",
                        edge_name(edge.kind),
                        target.map(|t| t.fqn.as_str()).unwrap_or(edge.to.as_str()),
                        rel_link(page, &target_page)
                    );
                }
                None => {
                    let _ = writeln!(
                        out,
                        "- {} → `{}`{marker}",
                        edge_name(edge.kind),
                        edge.to
                    );
                }
            }
        }
        out.push('\n');
    }

    let children: Vec<&Node> = node
        .children
        .iter()
        .filter_map(|child| doc.get(child))
        .collect();

    let compact: Vec<&&Node> = children.iter().filter(|c| is_compact(c.kind.as_str())).collect();
    if !compact.is_empty() {
        out.push_str("**Fields and variants**\n\n");
        for child in compact {
            let location = child
                .span
                .as_ref()
                .map(|span| span.location())
                .unwrap_or_default();
            let _ = writeln!(out, "- `{}` — `{}`", child.name, location);
        }
        out.push('\n');
    }

    for child in &children {
        if is_compact(child.kind.as_str()) || is_module(child) {
            continue;
        }
        render_node(out, doc, child, depth + 1, page, sources, out_rel, true);
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant render_node as render_node()
    participant repeat as repeat
    participant push as push
    participant push_str as push_str
    participant count as count
    participant get as get
    participant collect as collect
    participant unwrap_or_default as unwrap_or_default
    alt emit_heading
    render_node-->>repeat: ' '.repeat(depth.min(6))
    render_node-->>render_node: writeln!(out, '{hashes} `{}`\n', node.name)
    end
    render_node-->>render_node: writeln!(out, '| | |')
    render_node-->>render_node: writeln!(out, '|---|---|')
    render_node-->>render_node: writeln!(out, '| kind | `{}` |', node.kind)
    alt let Some(sig) = &node.signature
    render_node-->>render_node: writeln!(out, '| signature | `{}` |', cell(sig))
    end
    alt let Some(span) = &node.span
    render_node-->>render_node: writeln!(out, '| location | {} |', source_link(page, out_rel, span))
    end
    render_node-->>render_node: writeln!( out, '| provenance | {} · confidence {:.2} |', source_name(...
    render_node-->>push: out.push('\n')
    alt let Some(docs) = &node.docs
    loop docs.lines()
    render_node-->>render_node: writeln!(out, '> {line}')
    end
    render_node-->>push: out.push('\n')
    end
    alt let (Some(span), Some(sources)) = (&node.span, sources)
    alt let Some(text) = slice(sources, span)
    render_node-->>push_str: out.push_str('```rust\n')
    render_node-->>push_str: out.push_str(text.trim_end())
    render_node-->>push_str: out.push_str('\n```\n\n')
    end
    end
    alt let Some(process) = &node.process
    render_node-->>push_str: out.push_str('```mermaid\n')
    render_node-->>push_str: out.push_str(&sequence(process))
    render_node-->>push_str: out.push_str('```\n\n')
    render_node-->>count: process.steps.iter().filter(|step| step.observed).count()
    render_node-->>render_node: writeln!( out, '{} steps · {} observed · {} possible only\n', process...
    end
    alt !node.edges.is_empty()
    render_node-->>push_str: out.push_str('**Edges**\n\n')
    loop &node.edges
    alt edge.resolved
    render_node-->>render_node: ''
    else else
    render_node-->>render_node: ' (unresolved)'
    end
    render_node-->>get: doc.get(&edge.to)
    alt match target.and_then(|t| page_of_node(doc, t))
    render_node-->>render_node: writeln!( out, '- {} → [`{}`]({}){marker}', edge_name(edge.kind), tar...
    else None
    render_node-->>render_node: writeln!( out, '- {} → `{}`{marker}', edge_name(edge.kind), edge.to )
    end
    end
    render_node-->>push: out.push('\n')
    end
    render_node-->>collect: node .children .iter() .filter_map(|child| doc.get(child)) .collect()
    render_node-->>collect: children.iter().filter(|c| is_compact(c.kind.as_str())).collect()
    alt !compact.is_empty()
    render_node-->>push_str: out.push_str('**Fields and variants**\n\n')
    loop compact
    render_node-->>unwrap_or_default: child .span .as_ref() .map(|span| span.location()) .unwrap_or_default()
    render_node-->>render_node: writeln!(out, '- `{}` — `{}`', child.name, location)
    end
    render_node-->>push: out.push('\n')
    end
    loop &children
    alt is_compact(child.kind.as_str()) || is_module(child)
    render_node-->>render_node: continue
    end
    render_node-->>render_node: render_node(out, doc, child, depth + 1, page, sources, out_rel, true)
    end
```

34 steps · 0 observed · 34 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)
- calls → [`robby_render::tree::render_node`](render-node.md)

