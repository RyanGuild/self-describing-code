# `robby_render::view::execution::from_code_path`

[Self-model](../../../README.md) · [view::execution](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn from_code_path(doc: &Document, path: &CodePath) -> Option<Process>` |
| location | [`crates/robby-render/src/view/execution.rs:22`](../../../../../crates/robby-render/src/view/execution.rs#L22) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn from_code_path(doc: &Document, path: &CodePath) -> Option<Process> {
    let entry = doc.get(&path.entry)?;
    let mut process = Process::new();

    add_callable(&mut process, entry);

    let edges: Vec<_> = path.edges.iter().take(MAX_EDGES).collect();
    if edges.is_empty() {
        // No outbound path — fall back to the node's own process if any.
        return entry.process.clone().filter(|p| !p.steps.is_empty());
    }

    for edge in edges {
        let Some(from) = doc.get(&edge.from) else {
            continue;
        };
        let Some(to) = doc.get(&edge.to) else {
            continue;
        };
        add_callable(&mut process, from);

        let from_id = participant_id(from);
        if let Some(type_label) = owning_type_label(doc, to) {
            let type_id = alias(&type_label);
            process.participant(
                type_id.clone(),
                type_label,
                Kind::from(kind::code::STRUCT),
            );
            process.step(Step {
                from: from_id,
                to: type_id,
                label: to.name.clone(),
                kind: StepKind::Call,
                observed: false,
                span: None,
                blocks: Vec::new(),
            });
        } else {
            add_callable(&mut process, to);
            process.step(Step {
                from: from_id,
                to: participant_id(to),
                label: to.name.clone(),
                kind: StepKind::Call,
                observed: false,
                span: None,
                blocks: Vec::new(),
            });
        }
    }

    if process.steps.is_empty() {
        return None;
    }
    Some(process)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant from_code_path as from_code_path()
    participant get as get
    participant Process__new as Process::new
    participant add_callable as add_callable
    participant iter as iter
    participant take as take
    participant collect as collect
    participant is_empty as is_empty
    participant clone as clone
    participant filter as filter
    participant participant_id as participant_id
    participant owning_type_label as owning_type_label
    participant alias as alias
    participant Kind__from as Kind::from
    participant participant as participant
    participant Vec__new as Vec::new
    participant step as step
    participant Some as Some
    from_code_path-->>get: doc.get(&path.entry)
    from_code_path-->>from_code_path: ? (propagate)
    from_code_path-->>Process__new: Process::new()
    from_code_path-->>add_callable: add_callable(&mut process, entry)
    from_code_path-->>iter: path.edges.iter()
    from_code_path-->>take: path.edges.iter().take(MAX_EDGES)
    from_code_path-->>collect: path.edges.iter().take(MAX_EDGES).collect()
    from_code_path-->>is_empty: edges.is_empty()
    alt edges.is_empty()
    from_code_path-->>clone: entry.process.clone()
    from_code_path-->>is_empty: p.steps.is_empty()
    from_code_path-->>filter: entry.process.clone().filter(|p| !p.steps.is_empty())
    from_code_path-->>from_code_path: return
    end
    loop edges
    from_code_path-->>get: doc.get(&edge.from)
    from_code_path-->>get: doc.get(&edge.to)
    from_code_path-->>add_callable: add_callable(&mut process, from)
    from_code_path-->>participant_id: participant_id(from)
    from_code_path-->>owning_type_label: owning_type_label(doc, to)
    alt let Some(type_label) = owning_type_label(doc, to)
    from_code_path-->>alias: alias(&type_label)
    from_code_path-->>clone: type_id.clone()
    from_code_path-->>Kind__from: Kind::from(kind::code::STRUCT)
    from_code_path-->>participant: process.participant( type_id.clone(), type_label, Kind::from(kind::co...
    from_code_path-->>clone: to.name.clone()
    from_code_path-->>Vec__new: Vec::new()
    from_code_path-->>step: process.step(Step { from: from_id, to: type_id, label: to.name.clone(...
    else else
    from_code_path-->>add_callable: add_callable(&mut process, to)
    from_code_path-->>participant_id: participant_id(to)
    from_code_path-->>clone: to.name.clone()
    from_code_path-->>Vec__new: Vec::new()
    from_code_path-->>step: process.step(Step { from: from_id, to: participant_id(to), label: to....
    end
    end
    from_code_path-->>is_empty: process.steps.is_empty()
    alt process.steps.is_empty()
    from_code_path-->>from_code_path: return
    end
    from_code_path-->>Some: Some(process)
```

32 steps · 0 observed · 32 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](../../../robby_ir/model/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::view::execution::add_callable`](add-callable.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../links/index.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../links/index.md)
- calls → [`robby_ir::model::impl Document::get`](../../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::get`](../../../robby_ir/model/index.md)
- calls → [`robby_render::view::execution::add_callable`](add-callable.md)
- calls → [`robby_render::view::execution::participant_id`](participant-id.md)
- calls → [`robby_render::view::execution::owning_type_label`](owning-type-label.md)
- calls → [`robby_render::util::alias`](../../util/alias.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../../robby_ir/kind/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../../robby_ir/process/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::test_render::step`](../../test_render/step.md)
- calls → [`robby_render::view::execution::add_callable`](add-callable.md)
- calls → [`robby_render::view::execution::participant_id`](participant-id.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::test_render::step`](../../test_render/step.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../links/index.md)

## References

- calls → [`robby_ir::kind`](../../../robby_ir/kind/index.md) — from
- calls → [`robby_ir::model`](../../../robby_ir/model/index.md) — get
- calls → [`robby_ir::process`](../../../robby_ir/process/index.md) — participant
- calls → [`robby_render::test_render`](../../test_render/index.md) — step
- calls → [`robby_render::util`](../../util/index.md) — alias
- calls → [`robby_render::view::execution`](index.md) — add_callable, owning_type_label, participant_id
- calls → [`robby_render::view::links`](../links/index.md) — is_empty
- calls → [`robby_render::view::scene`](../scene/index.md) — new
- calls → [`robby_render::view::select`](../select/index.md) — collect

## Referenced by

- calls ← [`robby_render::view::execution::execution_process`](execution-process.md)

