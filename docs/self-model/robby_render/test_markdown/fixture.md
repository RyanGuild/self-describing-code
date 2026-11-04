# `robby_render::test_markdown::fixture`

[Self-model](../../README.md) · [test_markdown](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn fixture() -> (Document, BTreeMap<String, String>)` |
| location | [`crates/robby-render/tests/markdown.rs:12`](../../../../crates/robby-render/tests/markdown.rs#L12) |
| provenance | extractor · confidence 1.00 |

```rust
fn fixture() -> (Document, BTreeMap<String, String>) {
    let mut doc = Document::new("test");

    let mut module = Node::new(NodeId::from("rust://demo"), code::MODULE, "demo");
    module.kind = Kind::from(code::MODULE);
    doc.insert(module);

    let mut a = Node::new(NodeId::symbol("rust", "demo::a"), code::FUNCTION, "demo::a");
    a.parent = Some(NodeId::from("rust://demo"));
    a.span = Some(robby_ir::Span::new("src/demo.rs", SOURCE, 0, 28));
    a.signature = Some("fn a()".to_owned());
    a.docs = Some("Does a thing.".to_owned());
    a.provenance.source = Source::Extractor;

    let mut process = Process::new();
    process.participant("caller", "a", Kind::from(code::FUNCTION));
    process.participant("b", "b", Kind::from(code::FUNCTION));
    process.step(Step {
        from: "caller".to_owned(),
        to: "b".to_owned(),
        label: "b()".to_owned(),
        kind: StepKind::Call,
        observed: true,
        span: None,
        blocks: Vec::new(),
    });
    a.process = Some(process);
    doc.insert(a);

    let mut sources = BTreeMap::new();
    sources.insert("src/demo.rs".to_owned(), SOURCE.to_owned());

    (doc, sources)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant fixture as fixture()
    participant Document__new as Document::new
    participant NodeId__from as NodeId::from
    participant Node__new as Node::new
    participant Kind__from as Kind::from
    participant insert as insert
    participant NodeId__symbol as NodeId::symbol
    participant Some as Some
    participant robby_ir__Span__new as robby_ir::Span::new
    participant to_owned as to_owned
    participant Process__new as Process::new
    participant participant as participant
    participant Vec__new as Vec::new
    participant step as step
    participant BTreeMap__new as BTreeMap::new
    fixture-->>Document__new: Document::new('test')
    fixture-->>NodeId__from: NodeId::from('rust://demo')
    fixture-->>Node__new: Node::new(NodeId::from('rust://demo'), code::MODULE, 'demo')
    fixture-->>Kind__from: Kind::from(code::MODULE)
    fixture-->>insert: doc.insert(module)
    fixture-->>NodeId__symbol: NodeId::symbol('rust', 'demo::a')
    fixture-->>Node__new: Node::new(NodeId::symbol('rust', 'demo::a'), code::FUNCTION, 'demo::a')
    fixture-->>NodeId__from: NodeId::from('rust://demo')
    fixture-->>Some: Some(NodeId::from('rust://demo'))
    fixture-->>robby_ir__Span__new: robby_ir::Span::new('src/demo.rs', SOURCE, 0, 28)
    fixture-->>Some: Some(robby_ir::Span::new('src/demo.rs', SOURCE, 0, 28))
    fixture-->>to_owned: 'fn a()'.to_owned()
    fixture-->>Some: Some('fn a()'.to_owned())
    fixture-->>to_owned: 'Does a thing.'.to_owned()
    fixture-->>Some: Some('Does a thing.'.to_owned())
    fixture-->>Process__new: Process::new()
    fixture-->>Kind__from: Kind::from(code::FUNCTION)
    fixture-->>participant: process.participant('caller', 'a', Kind::from(code::FUNCTION))
    fixture-->>Kind__from: Kind::from(code::FUNCTION)
    fixture-->>participant: process.participant('b', 'b', Kind::from(code::FUNCTION))
    fixture-->>to_owned: 'caller'.to_owned()
    fixture-->>to_owned: 'b'.to_owned()
    fixture-->>to_owned: 'b()'.to_owned()
    fixture-->>Vec__new: Vec::new()
    fixture-->>step: process.step(Step { from: 'caller'.to_owned(), to: 'b'.to_owned(), la...
    fixture-->>Some: Some(process)
    fixture-->>insert: doc.insert(a)
    fixture-->>BTreeMap__new: BTreeMap::new()
    fixture-->>to_owned: 'src/demo.rs'.to_owned()
    fixture-->>to_owned: SOURCE.to_owned()
    fixture-->>insert: sources.insert('src/demo.rs'.to_owned(), SOURCE.to_owned())
```

31 steps · 0 observed · 31 possible only

**Edges**

- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../robby_ir/process/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../robby_ir/process/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::test_render::step`](../test_render/step.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)

## References

- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — from
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — insert, symbol
- calls → [`robby_ir::process`](../../robby_ir/process/index.md) — participant
- calls → [`robby_render::report`](../report/index.md) — new
- calls → [`robby_render::test_render`](../test_render/index.md) — step

## Referenced by

- calls ← [`robby_render::test_markdown::fences_are_balanced`](fences-are-balanced.md)
- calls ← [`robby_render::test_markdown::report_embeds_source_and_diagram_together`](report-embeds-source-and-diagram-together.md)
- calls ← [`robby_render::test_markdown::report_is_byte_stable`](report-is-byte-stable.md)
- calls ← [`robby_render::test_markdown::report_without_sources_still_renders`](report-without-sources-still-renders.md)

