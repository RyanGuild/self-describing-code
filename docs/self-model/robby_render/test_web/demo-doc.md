# `robby_render::test_web::demo_doc`

[Self-model](../../README.md) · [test_web](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn demo_doc() -> (Document, BTreeMap<String, String>)` |
| location | [`crates/robby-render/tests/web.rs:10`](../../../../crates/robby-render/tests/web.rs#L10) |
| provenance | extractor · confidence 1.00 |

```rust
fn demo_doc() -> (Document, BTreeMap<String, String>) {
    let mut doc = Document::new("test");
    let mut sources = BTreeMap::new();
    sources.insert(
        "crates/demo/src/lib.rs".to_owned(),
        "fn a() {}\nfn b() { a(); }\n".to_owned(),
    );

    let mut krate = Node::new(NodeId::from("rust://demo"), code::CRATE, "demo");
    krate.kind = Kind::from(code::CRATE);
    doc.insert(krate);

    let mut left = Node::new(
        NodeId::symbol("rust", "demo::left"),
        code::MODULE,
        "demo::left",
    );
    left.parent = Some(NodeId::from("rust://demo"));
    doc.insert(left);

    let mut right = Node::new(
        NodeId::symbol("rust", "demo::right"),
        code::MODULE,
        "demo::right",
    );
    right.parent = Some(NodeId::from("rust://demo"));
    doc.insert(right);

    let mut a = Node::new(
        NodeId::symbol("rust", "demo::left::a"),
        code::FUNCTION,
        "demo::left::a",
    );
    a.parent = Some(NodeId::symbol("rust", "demo::left"));
    a.signature = Some("fn a()".to_owned());
    a.span = Some(Span::new(
        "crates/demo/src/lib.rs",
        sources.get("crates/demo/src/lib.rs").unwrap(),
        0,
        9,
    ));
    let mut proc = Process::new();
    proc.participant("a", "a", Kind::from(code::FUNCTION));
    proc.step(Step {
        from: "a".to_owned(),
        to: "a".to_owned(),
        label: "body".to_owned(),
        kind: StepKind::Note,
        observed: false,
        span: None,
        blocks: vec![],
    });
    a.process = Some(proc);
    doc.insert(a);

    let mut b = Node::new(
        NodeId::symbol("rust", "demo::right::b"),
        code::FUNCTION,
        "demo::right::b",
    );
    b.parent = Some(NodeId::symbol("rust", "demo::right"));
    b.edges.push(Edge::new(
        NodeId::symbol("rust", "demo::left::a"),
        EdgeKind::Calls,
    ));
    let mut unresolved = Edge::new(NodeId::from("rust://missing::gone"), EdgeKind::Calls);
    unresolved.resolved = false;
    b.edges.push(unresolved);
    b.span = Some(Span::new(
        "crates/demo/src/lib.rs",
        sources.get("crates/demo/src/lib.rs").unwrap(),
        10,
        26,
    ));
    doc.insert(b);

    (doc, sources)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant demo_doc as demo_doc()
    participant Document__new as Document::new
    participant BTreeMap__new as BTreeMap::new
    participant to_owned as to_owned
    participant insert as insert
    participant NodeId__from as NodeId::from
    participant Node__new as Node::new
    participant Kind__from as Kind::from
    participant NodeId__symbol as NodeId::symbol
    participant Some as Some
    participant get as get
    participant unwrap as unwrap
    participant Span__new as Span::new
    participant Process__new as Process::new
    participant participant as participant
    participant step as step
    participant Edge__new as Edge::new
    participant push as push
    demo_doc-->>Document__new: Document::new('test')
    demo_doc-->>BTreeMap__new: BTreeMap::new()
    demo_doc-->>to_owned: 'crates/demo/src/lib.rs'.to_owned()
    demo_doc-->>to_owned: 'fn a() {}\nfn b() { a() }\n'.to_owned()
    demo_doc-->>insert: sources.insert( 'crates/demo/src/lib.rs'.to_owned(), 'fn a() {}\nfn b...
    demo_doc-->>NodeId__from: NodeId::from('rust://demo')
    demo_doc-->>Node__new: Node::new(NodeId::from('rust://demo'), code::CRATE, 'demo')
    demo_doc-->>Kind__from: Kind::from(code::CRATE)
    demo_doc-->>insert: doc.insert(krate)
    demo_doc-->>NodeId__symbol: NodeId::symbol('rust', 'demo::left')
    demo_doc-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::left'), code::MODULE, 'demo:...
    demo_doc-->>NodeId__from: NodeId::from('rust://demo')
    demo_doc-->>Some: Some(NodeId::from('rust://demo'))
    demo_doc-->>insert: doc.insert(left)
    demo_doc-->>NodeId__symbol: NodeId::symbol('rust', 'demo::right')
    demo_doc-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::right'), code::MODULE, 'demo...
    demo_doc-->>NodeId__from: NodeId::from('rust://demo')
    demo_doc-->>Some: Some(NodeId::from('rust://demo'))
    demo_doc-->>insert: doc.insert(right)
    demo_doc-->>NodeId__symbol: NodeId::symbol('rust', 'demo::left::a')
    demo_doc-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::left::a'), code::FUNCTION, '...
    demo_doc-->>NodeId__symbol: NodeId::symbol('rust', 'demo::left')
    demo_doc-->>Some: Some(NodeId::symbol('rust', 'demo::left'))
    demo_doc-->>to_owned: 'fn a()'.to_owned()
    demo_doc-->>Some: Some('fn a()'.to_owned())
    demo_doc-->>get: sources.get('crates/demo/src/lib.rs')
    demo_doc-->>unwrap: sources.get('crates/demo/src/lib.rs').unwrap()
    demo_doc-->>Span__new: Span::new( 'crates/demo/src/lib.rs', sources.get('crates/demo/src/lib...
    demo_doc-->>Some: Some(Span::new( 'crates/demo/src/lib.rs', sources.get('crates/demo/sr...
    demo_doc-->>Process__new: Process::new()
    demo_doc-->>Kind__from: Kind::from(code::FUNCTION)
    demo_doc-->>participant: proc.participant('a', 'a', Kind::from(code::FUNCTION))
    demo_doc-->>to_owned: 'a'.to_owned()
    demo_doc-->>to_owned: 'a'.to_owned()
    demo_doc-->>to_owned: 'body'.to_owned()
    demo_doc-->>demo_doc: vec![]
    demo_doc-->>step: proc.step(Step { from: 'a'.to_owned(), to: 'a'.to_owned(), label: 'bo...
    demo_doc-->>Some: Some(proc)
    demo_doc-->>insert: doc.insert(a)
    demo_doc-->>NodeId__symbol: NodeId::symbol('rust', 'demo::right::b')
    demo_doc-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::right::b'), code::FUNCTION, ...
    demo_doc-->>NodeId__symbol: NodeId::symbol('rust', 'demo::right')
    demo_doc-->>Some: Some(NodeId::symbol('rust', 'demo::right'))
    demo_doc-->>NodeId__symbol: NodeId::symbol('rust', 'demo::left::a')
    demo_doc-->>Edge__new: Edge::new( NodeId::symbol('rust', 'demo::left::a'), EdgeKind::Calls, )
    demo_doc-->>push: b.edges.push(Edge::new( NodeId::symbol('rust', 'demo::left::a'), Edge...
    demo_doc-->>NodeId__from: NodeId::from('rust://missing::gone')
    demo_doc-->>Edge__new: Edge::new(NodeId::from('rust://missing::gone'), EdgeKind::Calls)
    demo_doc-->>push: b.edges.push(unresolved)
    demo_doc-->>get: sources.get('crates/demo/src/lib.rs')
    demo_doc-->>unwrap: sources.get('crates/demo/src/lib.rs').unwrap()
    demo_doc-->>Span__new: Span::new( 'crates/demo/src/lib.rs', sources.get('crates/demo/src/lib...
    demo_doc-->>Some: Some(Span::new( 'crates/demo/src/lib.rs', sources.get('crates/demo/sr...
    demo_doc-->>insert: doc.insert(b)
```

54 steps · 0 observed · 54 possible only

**Edges**

- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../robby_ir/process/index.md)
- calls → [`robby_render::test_render::step`](../test_render/step.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::report::impl Report::push`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::report::impl Report::push`](../report/index.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)

## References

- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — from
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — insert, symbol
- calls → [`robby_ir::process`](../../robby_ir/process/index.md) — participant
- calls → [`robby_ir::run`](../../robby_ir/run/index.md) — get
- calls → [`robby_render::report`](../report/index.md) — new, push
- calls → [`robby_render::test_render`](../test_render/index.md) — step

## Referenced by

- calls ← [`robby_render::test_web::child_world_rects_lie_inside_parent_world`](child-world-rects-lie-inside-parent-world.md)
- calls ← [`robby_render::test_web::every_child_and_resolved_portal_is_a_frame`](every-child-and-resolved-portal-is-a-frame.md)
- calls ← [`robby_render::test_web::leaf_payload_includes_source_and_process`](leaf-payload-includes-source-and-process.md)
- calls ← [`robby_render::test_web::scene_json_is_byte_identical_across_runs`](scene-json-is-byte-identical-across-runs.md)
- calls ← [`robby_render::test_web::sibling_local_rects_do_not_overlap`](sibling-local-rects-do-not-overlap.md)
- calls ← [`robby_render::test_web::sibling_world_rects_do_not_collapse_onto_each_other`](sibling-world-rects-do-not-collapse-onto-each-other.md)
- calls ← [`robby_render::test_web::unresolved_portal_has_no_zoom_target`](unresolved-portal-has-no-zoom-target.md)
- calls ← [`robby_render::test_web::write_site_emits_run_records`](write-site-emits-run-records.md)
- calls ← [`robby_render::test_web::write_site_emits_scene_and_frames`](write-site-emits-scene-and-frames.md)

