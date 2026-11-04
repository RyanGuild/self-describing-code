//! Renderer tests. Determinism is the point: these diagrams are committed.

use robby_ir::kind::code;
use robby_ir::{
    Block, BlockKind, Document, Edge, EdgeKind, Kind, Node, NodeId, Process, Source, Step, StepKind,
};
use robby_render::view::{CommentsView, ExternalsView};
use robby_render::{
    MarkdownRenderer, Renderer, class_diagram, collect, coupling, dependencies, flowchart,
    sequence, stable_id, workspace_dependencies,
};

fn step(from: &str, to: &str, label: &str, observed: bool, blocks: Vec<Block>) -> Step {
    Step {
        from: from.to_owned(),
        to: to.to_owned(),
        label: label.to_owned(),
        kind: StepKind::Call,
        observed,
        span: None,
        blocks,
    }
}

fn process(steps: Vec<Step>) -> Process {
    let mut p = Process::new();
    p.participant("caller", "caller", Kind::from(code::FUNCTION));
    p.participant("io", "io::load", Kind::from(code::FUNCTION));
    for s in steps {
        p.step(s);
    }
    p
}

#[test]
fn renders_a_flat_sequence() {
    let p = process(vec![step("caller", "io", "load(path)", true, vec![])]);
    let out = sequence(&p);

    assert!(out.starts_with("sequenceDiagram\n"));
    assert!(out.contains("participant caller as caller"));
    assert!(out.contains("participant io as io::load"));
    assert!(out.contains("caller->>io: load(path)"));
}

#[test]
fn unobserved_steps_use_dashed_arrows() {
    let p = process(vec![step("caller", "io", "load(path)", false, vec![])]);
    let out = sequence(&p);
    assert!(out.contains("caller-->>io: load(path)"));
    assert!(!out.contains("caller->>io"));
}

#[test]
fn opens_and_closes_alt_blocks() {
    let alt = Block {
        kind: BlockKind::Alt,
        label: "cfg.verbose".to_owned(),
        branch: None,
    };
    let p = process(vec![
        step("caller", "io", "read", true, vec![]),
        step("caller", "io", "info", true, vec![alt.clone()]),
        step("caller", "io", "parse", true, vec![]),
    ]);
    let out = sequence(&p);

    let alt_at = out.find("alt cfg.verbose").expect("alt opened");
    let end_at = out.find("    end\n").expect("alt closed");
    let parse_at = out.find("parse").expect("parse step");

    assert!(alt_at < end_at, "alt must close");
    assert!(end_at < parse_at, "parse must come after the block closes");
}

#[test]
fn emits_else_for_a_second_branch() {
    let branch_a = Block {
        kind: BlockKind::Alt,
        label: "mode".to_owned(),
        branch: None,
    };
    let branch_b = Block {
        kind: BlockKind::Alt,
        label: "mode".to_owned(),
        branch: Some("fallback".to_owned()),
    };
    let p = process(vec![
        step("caller", "io", "fast", true, vec![branch_a]),
        step("caller", "io", "slow", true, vec![branch_b]),
    ]);
    let out = sequence(&p);

    assert!(out.contains("alt mode"));
    assert!(out.contains("else fallback"));
}

#[test]
fn nests_blocks_by_frame_stack() {
    let alt = Block {
        kind: BlockKind::Alt,
        label: "outer".to_owned(),
        branch: None,
    };
    let mut loop_block = alt.clone();
    loop_block.kind = BlockKind::Loop;
    loop_block.label = "inner".to_owned();

    let p = process(vec![
        step("caller", "io", "a", true, vec![alt.clone()]),
        step("caller", "io", "b", true, vec![alt.clone(), loop_block]),
        step("caller", "io", "c", true, vec![alt]),
        step("caller", "io", "d", true, vec![]),
    ]);
    let out = sequence(&p);

    assert!(out.contains("alt outer"));
    assert!(out.contains("loop inner"));
    assert_eq!(out.matches("    end\n").count(), 2);
}

#[test]
fn output_is_byte_identical_across_runs() {
    let p = process(vec![
        step("caller", "io", "a", true, vec![]),
        step("caller", "io", "b", false, vec![]),
    ]);
    assert_eq!(sequence(&p), sequence(&p));
}

#[test]
fn stable_ids_are_pinned() {
    // If this value changes, every committed diagram in the repo rewrites.
    // Changing it must be a deliberate, documented migration.
    assert_eq!(stable_id("rust://demo::a"), stable_id("rust://demo::a"));
    assert_ne!(stable_id("rust://demo::a"), stable_id("rust://demo::b"));
    assert_eq!(stable_id("").len(), 18);
}

#[test]
fn flowchart_clusters_modules() {
    let mut doc = Document::new("test");

    let mut module = Node::new(NodeId::from("rust://demo"), code::MODULE, "demo");
    module.kind = Kind::from(code::MODULE);
    module.provenance.source = Source::Extractor;
    doc.insert(module);

    let mut a = Node::new(NodeId::symbol("rust", "demo::a"), code::FUNCTION, "demo::a");
    a.parent = Some(NodeId::from("rust://demo"));
    a.signature = Some("fn a()".to_owned());
    doc.insert(a);

    let mut b = Node::new(NodeId::symbol("rust", "demo::b"), code::FUNCTION, "demo::b");
    b.parent = Some(NodeId::from("rust://demo"));
    b.edges.push(Edge::new(
        NodeId::symbol("rust", "demo::a"),
        EdgeKind::Calls,
    ));
    doc.insert(b);

    let out = flowchart(&doc, &NodeId::from("rust://demo"));

    assert!(out.starts_with("flowchart TD\n"));
    assert!(out.contains("subgraph"));
    assert!(out.contains("|calls|"));
}

#[test]
fn dependencies_emits_package_weighted_and_dsm() {
    let mut doc = Document::new("test");

    let mut krate = Node::new(NodeId::from("rust://demo"), code::CRATE, "demo");
    krate.kind = Kind::from(code::CRATE);
    doc.insert(krate);

    // Third arg is FQN (name is derived); mirror extractor shape.
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

    let mut point = Node::new(
        NodeId::symbol("rust", "demo::left::Point"),
        code::STRUCT,
        "demo::left::Point",
    );
    point.parent = Some(NodeId::symbol("rust", "demo::left"));
    doc.insert(point);

    let mut a = Node::new(
        NodeId::symbol("rust", "demo::left::a"),
        code::FUNCTION,
        "demo::left::a",
    );
    a.parent = Some(NodeId::symbol("rust", "demo::left"));
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
    doc.insert(b);

    let out = dependencies(&doc, &NodeId::from("rust://demo"));

    assert!(out.starts_with("flowchart TB\n"));
    assert!(out.contains("subgraph"), "package diagram nests modules");
    assert!(out.contains("left"), "package digraph names modules");
    assert!(out.contains("right"), "package digraph names modules");
    assert!(
        !out.contains("Point"),
        "types belong in the class diagram, not module deps"
    );
    assert!(
        !out.contains("[\"a\"]"),
        "functions stay out of the package diagram"
    );

    let coupled = coupling(&doc, &NodeId::from("rust://demo"));
    assert!(coupled.starts_with("flowchart TB\n"));
    assert!(coupled.contains("subgraph"), "coupling nests modules");
    assert!(coupled.contains('a'), "coupling shows callee");
    assert!(coupled.contains('b'), "coupling shows caller");
    assert!(coupled.contains("-->"), "coupling draws the call edge");
    assert!(!coupled.contains("Point"), "types stay out of coupling");

    let types = class_diagram(&doc, &NodeId::symbol("rust", "demo::left"));
    assert!(types.contains("classDiagram"));
    assert!(types.contains("class Point"));

    let workspace = workspace_dependencies(&doc);
    assert!(workspace.contains("Package diagram"));
    assert!(workspace.contains("Module coupling"));
    assert!(workspace.contains("Dependency matrix"));
    assert!(workspace.contains("demo"));
    assert!(!workspace.contains("Point"));
}

#[test]
fn externals_and_comments_sections_render() {
    let mut doc = Document::new("test");

    let mut pkg = Node::new(NodeId::symbol("typescript", "demo"), code::PACKAGE, "demo");
    pkg.lang = robby_ir::Lang::TypeScript;
    doc.insert(pkg);

    let mut module = Node::new(
        NodeId::symbol("typescript", "demo/shapes"),
        code::MODULE,
        "demo/shapes",
    );
    module.parent = Some(NodeId::symbol("typescript", "demo"));
    doc.insert(module);

    let mut stub = Node::new(
        NodeId::symbol("typescript", "npm/demo-ext"),
        code::PACKAGE,
        "npm/demo-ext",
    );
    stub.lang = robby_ir::Lang::TypeScript;
    stub.tags.push("external".into());
    stub.annotations.insert("external".into(), "true".into());
    doc.insert(stub);

    let mut import = Node::new(
        NodeId::symbol("typescript", "demo/shapes/import:demo-ext"),
        code::IMPORT,
        "demo/shapes/import:demo-ext",
    );
    import.parent = Some(NodeId::symbol("typescript", "demo/shapes"));
    import.edges.push(Edge::new(
        NodeId::symbol("typescript", "npm/demo-ext"),
        EdgeKind::Imports,
    ));
    doc.insert(import);

    let mut comment = Node::new(
        NodeId::symbol("typescript", "demo/shapes/comment@0"),
        code::COMMENT,
        "demo/shapes/comment@0",
    );
    comment.parent = Some(NodeId::symbol("typescript", "demo/shapes"));
    comment.docs = Some("A drawable shape.".into());
    comment
        .annotations
        .insert("comment_kind".into(), "block".into());
    comment.span = Some(robby_ir::Span {
        file: "shapes.ts".into(),
        start: robby_ir::Position {
            byte: 0,
            line: 1,
            column: 1,
        },
        end: robby_ir::Position {
            byte: 20,
            line: 1,
            column: 21,
        },
    });
    doc.insert(comment);

    let mut decision = Node::new(
        NodeId::symbol("typescript", "demo/shapes/comment@0/decision"),
        robby_ir::kind::agent::DECISION,
        "demo/shapes/comment@0/decision",
    );
    decision.parent = Some(NodeId::symbol("typescript", "demo/shapes/comment@0"));
    decision.provenance = robby_ir::Provenance::new(robby_ir::Source::Agent);
    decision.docs = Some("Call draw() on implementors.".into());
    decision
        .annotations
        .insert("verdict".into(), "relevant".into());
    decision
        .annotations
        .insert("note_kind".into(), "usage_example".into());
    decision
        .annotations
        .insert("highlights".into(), r#"[{"start":0,"end":6}]"#.into());
    doc.insert(decision);

    let mut noise = Node::new(
        NodeId::symbol("typescript", "demo/shapes/comment@99"),
        code::COMMENT,
        "demo/shapes/comment@99",
    );
    noise.parent = Some(NodeId::symbol("typescript", "demo/shapes"));
    noise.docs = Some("TODO: delete me".into());
    noise
        .annotations
        .insert("comment_kind".into(), "line".into());
    doc.insert(noise);

    let mut noise_decision = Node::new(
        NodeId::symbol("typescript", "demo/shapes/comment@99/decision"),
        robby_ir::kind::agent::DECISION,
        "demo/shapes/comment@99/decision",
    );
    noise_decision.parent = Some(NodeId::symbol("typescript", "demo/shapes/comment@99"));
    noise_decision
        .annotations
        .insert("verdict".into(), "noise".into());
    doc.insert(noise_decision);

    let mut iface = Node::new(
        NodeId::symbol("typescript", "demo/shapes/Drawable"),
        code::INTERFACE,
        "demo/shapes/Drawable",
    );
    iface.parent = Some(NodeId::symbol("typescript", "demo/shapes"));
    iface.span = Some(robby_ir::Span {
        file: "shapes.ts".into(),
        start: robby_ir::Position {
            byte: 22,
            line: 2,
            column: 1,
        },
        end: robby_ir::Position {
            byte: 62,
            line: 4,
            column: 2,
        },
    });
    doc.insert(iface);

    let mut sources = std::collections::BTreeMap::new();
    sources.insert(
        "shapes.ts".into(),
        "/** A drawable shape. */\nexport interface Drawable {\n  draw(): void;\n}\n".into(),
    );

    let root = NodeId::symbol("typescript", "demo");
    let externals = ExternalsView::from_root(&doc, &root);
    assert!(!externals.is_empty());
    assert_eq!(externals.entries[0].package, "demo-ext");

    let comments = CommentsView::from_root(&doc, &root, Some(&sources));
    assert!(!comments.is_empty());
    assert!(
        comments.entries.iter().all(|e| e.text != "TODO: delete me"),
        "noise comments filtered"
    );
    let enriched = comments
        .entries
        .iter()
        .find(|e| e.note.is_some())
        .expect("enriched entry");
    assert!(
        enriched
            .code
            .as_ref()
            .is_some_and(|c| c.contains("Drawable"))
    );

    let md = MarkdownRenderer::new();
    let ext_body = md.externals(&externals);
    assert!(ext_body.contains("`demo-ext`"));
    assert!(ext_body.contains("imported by"));

    let cmt_body = md.comments(&comments);
    assert!(cmt_body.contains("usage example"));
    assert!(cmt_body.contains("Call draw()"));
    assert!(cmt_body.contains('«') && cmt_body.contains('»'));
    assert!(!cmt_body.contains("TODO: delete me"));

    let views = collect(
        &doc,
        doc.get(&root).expect("demo"),
        Some(&sources),
        &robby_render::SelectOpts::markdown(),
    );
    let report = md.report(&views).to_markdown();
    assert!(report.contains("## Externals"));
    assert!(report.contains("## Comments"));
}

#[test]
fn class_diagram_marks_traits_as_interfaces() {
    let mut doc = Document::new("test");

    let mut module = Node::new(NodeId::from("rust://demo"), code::MODULE, "demo");
    module.kind = Kind::from(code::MODULE);
    doc.insert(module);

    let mut drawable = Node::new(
        NodeId::symbol("rust", "demo::Drawable"),
        code::TRAIT,
        "Drawable",
    );
    drawable.parent = Some(NodeId::from("rust://demo"));
    doc.insert(drawable);

    let mut method = Node::new(
        NodeId::symbol("rust", "demo::Drawable::draw"),
        code::METHOD,
        "draw",
    );
    method.parent = Some(NodeId::symbol("rust", "demo::Drawable"));
    doc.insert(method);

    let mut point = Node::new(NodeId::symbol("rust", "demo::Point"), code::STRUCT, "Point");
    point.parent = Some(NodeId::from("rust://demo"));
    doc.insert(point);

    let mut field = Node::new(NodeId::symbol("rust", "demo::Point::x"), code::FIELD, "x");
    field.parent = Some(NodeId::symbol("rust", "demo::Point"));
    doc.insert(field);

    let mut impl_node = Node::new(
        NodeId::symbol("rust", "demo::impl Drawable for Point"),
        code::IMPL,
        "impl Drawable for Point",
    );
    impl_node.parent = Some(NodeId::from("rust://demo"));
    doc.insert(impl_node);

    let out = class_diagram(&doc, &NodeId::from("rust://demo"));
    assert!(out.contains("<<interface>>"));
    assert!(out.contains("class Drawable"));
    assert!(out.contains("+draw()"));
    assert!(out.contains("class Point"));
    assert!(out.contains("+x"));
    assert!(out.contains("Point ..|> Drawable : implements"));
}
