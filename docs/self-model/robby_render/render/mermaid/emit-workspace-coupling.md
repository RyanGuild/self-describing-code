# `robby_render::render::mermaid::emit_workspace_coupling`

[Self-model](../../../README.md) · [render::mermaid](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn emit_workspace_coupling(out: &mut String, node: &CouplingModule, depth: usize)` |
| location | [`crates/robby-render/src/render/mermaid.rs:231`](../../../../../crates/robby-render/src/render/mermaid.rs#L231) |
| provenance | extractor · confidence 1.00 |

```rust
fn emit_workspace_coupling(out: &mut String, node: &CouplingModule, depth: usize) {
    let indent = "    ".repeat(depth);
    if node.children.is_empty() {
        let _ = writeln!(out, "{indent}{}[\"{}\"]", node.stable_id, node.name);
        return;
    }
    let _ = writeln!(
        out,
        "{indent}subgraph {}[\"{}\"]",
        node.stable_id, node.name
    );
    let _ = writeln!(out, "{indent}    direction TB");
    for child in &node.children {
        emit_workspace_coupling(out, child, depth + 1);
    }
    let _ = writeln!(out, "{indent}end");
}
```

```mermaid
sequenceDiagram
    autonumber
    participant emit_workspace_coupling as emit_workspace_coupling()
    participant repeat as repeat
    participant is_empty as is_empty
    emit_workspace_coupling-->>repeat: ' '.repeat(depth)
    emit_workspace_coupling-->>is_empty: node.children.is_empty()
    alt node.children.is_empty()
    emit_workspace_coupling-->>emit_workspace_coupling: writeln!(out, '{indent}{}[\'{}\']', node.stable_id, node.name)
    emit_workspace_coupling-->>emit_workspace_coupling: return
    end
    emit_workspace_coupling-->>emit_workspace_coupling: writeln!( out, '{indent}subgraph {}[\'{}\']', node.stable_id, node.na...
    emit_workspace_coupling-->>emit_workspace_coupling: writeln!(out, '{indent} direction TB')
    loop &node.children
    emit_workspace_coupling-->>emit_workspace_coupling: emit_workspace_coupling(out, child, depth + 1)
    end
    emit_workspace_coupling-->>emit_workspace_coupling: writeln!(out, '{indent}end')
```

8 steps · 0 observed · 8 possible only

**Edges**

- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../../view/links/index.md)
- calls → [`robby_render::render::mermaid::emit_workspace_coupling`](emit-workspace-coupling.md)

## References

- calls → [`robby_render::view::links`](../../view/links/index.md) — is_empty

## Referenced by

- calls ← [`robby_render::render::mermaid::impl Renderer for MermaidRenderer::coupling_deps`](index.md)

