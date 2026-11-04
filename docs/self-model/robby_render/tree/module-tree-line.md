# `robby_render::tree::module_tree_line`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn module_tree_line(doc: &Document, node: &Node, crate_dir: &str, depth: usize, out: &mut String)` |
| location | [`crates/robby-render/src/tree.rs:186`](../../../../crates/robby-render/src/tree.rs#L186) |
| provenance | extractor · confidence 1.00 |

```rust
fn module_tree_line(doc: &Document, node: &Node, crate_dir: &str, depth: usize, out: &mut String) {
    let Some(index) = module_index_path(node, crate_dir) else {
        return;
    };
    let indent = "  ".repeat(depth);
    let _ = writeln!(
        out,
        "{indent}- [`{}`]({}) — {} nodes",
        short_fqn(node, crate_dir),
        index.to_string_lossy().replace('\\', "/"),
        doc.subtree(&node.id).len()
    );
    for child in &node.children {
        let Some(child) = doc.get(child) else {
            continue;
        };
        if is_module(child) {
            module_tree_line(doc, child, crate_dir, depth + 1, out);
        }
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant module_tree_line as module_tree_line()
    participant module_index_path as module_index_path
    participant repeat as repeat
    participant get as get
    participant is_module as is_module
    module_tree_line-->>module_index_path: module_index_path(node, crate_dir)
    module_tree_line-->>repeat: ' '.repeat(depth)
    module_tree_line-->>module_tree_line: writeln!( out, '{indent}- [`{}`]({}) — {} nodes', short_fqn(node, cra...
    loop &node.children
    module_tree_line-->>get: doc.get(child)
    module_tree_line-->>is_module: is_module(child)
    alt is_module(child)
    module_tree_line-->>module_tree_line: module_tree_line(doc, child, crate_dir, depth + 1, out)
    end
    end
```

6 steps · 0 observed · 6 possible only

**Edges**

- calls → [`robby_render::tree::module_index_path`](module-index-path.md)
- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)
- calls → [`robby_render::tree::is_module`](is-module.md)
- calls → [`robby_render::tree::module_tree_line`](module-tree-line.md)

## References

- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — get
- calls → [`robby_render::tree`](index.md) — is_module, module_index_path

## Referenced by

- calls ← [`robby_render::tree::global_index`](global-index.md)

