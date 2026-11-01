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
    participant Node__new as Node::new
    participant insert as insert
    participant Process__new as Process::new
    participant participant as participant
    participant step as step
    participant BTreeMap__new as BTreeMap::new
    fixture-->>Document__new: Document::new('test')
    fixture-->>Node__new: Node::new(NodeId::from('rust://demo'), code::MODULE, 'demo')
    fixture-->>fixture: module.kind = Kind::from(code::MODULE)
    fixture-->>insert: doc.insert(module)
    fixture-->>Node__new: Node::new(NodeId::symbol('rust', 'demo::a'), code::FUNCTION, 'demo::a')
    fixture-->>fixture: a.parent = Some(NodeId::from('rust://demo'))
    fixture-->>fixture: a.span = Some(robby_ir::Span::new('src/demo.rs', SOURCE, 0, 28))
    fixture-->>fixture: a.signature = Some('fn a()'.to_owned())
    fixture-->>fixture: a.docs = Some('Does a thing.'.to_owned())
    fixture-->>fixture: a.provenance.source = Source::Extractor
    fixture-->>Process__new: Process::new()
    fixture-->>participant: process.participant('caller', 'a', Kind::from(code::FUNCTION))
    fixture-->>participant: process.participant('b', 'b', Kind::from(code::FUNCTION))
    fixture-->>step: process.step(Step { from: 'caller'.to_owned(), to: 'b'.to_owned(), la...
    fixture-->>fixture: a.process = Some(process)
    fixture-->>insert: doc.insert(a)
    fixture-->>BTreeMap__new: BTreeMap::new()
    fixture-->>insert: sources.insert('src/demo.rs'.to_owned(), SOURCE.to_owned())
    fixture-->>fixture: (doc, sources)
```

19 steps · 0 observed · 19 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)
- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)
- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../robby_ir/process/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../robby_ir/process/index.md)
- calls → [`robby_ir::process::impl Process::step`](../../robby_ir/process/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)

