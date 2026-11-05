//! Prezi scene + frame payload invariants.

use std::collections::BTreeMap;

use robby_ir::kind::code;
use robby_ir::{Document, Edge, EdgeKind, Kind, Node, NodeId, Process, Span, Step, StepKind};
use robby_render::view::{Scene, WORKSPACE_ID};
use robby_render::{FrameDetail, stable_id, write_site};

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

/// Two `main` entries that both call a shared callee (first parent wins).
fn entrypoints_demo_doc() -> Document {
    let mut doc = Document::new("test");

    let mut main_a = Node::new(
        NodeId::symbol("rust", "demo::bin_a::main"),
        code::FUNCTION,
        "demo::bin_a::main",
    );
    main_a.edges.push(Edge::new(
        NodeId::symbol("rust", "demo::shared"),
        EdgeKind::Calls,
    ));
    main_a.edges.push(Edge::new(
        NodeId::symbol("rust", "demo::helper"),
        EdgeKind::Calls,
    ));
    doc.insert(main_a);

    let mut helper = Node::new(
        NodeId::symbol("rust", "demo::helper"),
        code::FUNCTION,
        "demo::helper",
    );
    helper.edges.push(Edge::new(
        NodeId::symbol("rust", "demo::shared"),
        EdgeKind::Calls,
    ));
    doc.insert(helper);

    let shared = Node::new(
        NodeId::symbol("rust", "demo::shared"),
        code::FUNCTION,
        "demo::shared",
    );
    doc.insert(shared);

    let mut main_b = Node::new(
        NodeId::symbol("rust", "demo::bin_b::main"),
        code::FUNCTION,
        "demo::bin_b::main",
    );
    main_b.edges.push(Edge::new(
        NodeId::symbol("rust", "demo::shared"),
        EdgeKind::Calls,
    ));
    doc.insert(main_b);

    doc.rebuild_code_paths();
    doc
}

#[test]
fn scene_json_is_byte_identical_across_runs() {
    let (doc, _) = demo_doc();
    let a = Scene::from_document(&doc).to_json();
    let b = Scene::from_document(&doc).to_json();
    assert_eq!(a, b);
}

#[test]
fn every_child_and_resolved_portal_is_a_frame() {
    let (doc, _) = demo_doc();
    let scene = Scene::from_document(&doc);
    for frame in scene.frames.values() {
        for child in &frame.children {
            assert!(
                scene.frames.contains_key(child),
                "missing child frame {child}"
            );
        }
        for portal in &frame.portals {
            if let Some(target) = &portal.target {
                assert!(
                    scene.frames.contains_key(target),
                    "portal target {target} missing from scene"
                );
            }
        }
    }
    assert!(scene.frames.contains_key(&stable_id(WORKSPACE_ID)));
}

#[test]
fn child_world_rects_lie_inside_parent_world() {
    let (doc, _) = demo_doc();
    let scene = Scene::from_document(&doc);
    for frame in scene.frames.values() {
        for child_id in &frame.children {
            let child = &scene.frames[child_id];
            assert!(
                frame.world.contains(&child.world),
                "{} ({:?}) not inside {} ({:?})",
                child.stable_id,
                child.world,
                frame.stable_id,
                frame.world
            );
        }
    }
}

#[test]
fn sibling_local_rects_do_not_overlap() {
    let (doc, _) = demo_doc();
    let scene = Scene::from_document(&doc);
    for frame in scene.frames.values() {
        let locals: Vec<_> = frame
            .children
            .iter()
            .filter_map(|id| scene.frames[id].local_in_parent)
            .collect();
        for (i, a) in locals.iter().enumerate() {
            for b in locals.iter().skip(i + 1) {
                let ax2 = a.x + a.w;
                let ay2 = a.y + a.h;
                let bx2 = b.x + b.w;
                let by2 = b.y + b.h;
                let overlap = !(ax2 <= b.x || bx2 <= a.x || ay2 <= b.y || by2 <= a.y);
                assert!(
                    !overlap,
                    "siblings overlap in {} local grid: {:?} vs {:?}",
                    frame.fqn, a, b
                );
            }
        }
    }
}

#[test]
fn sibling_world_rects_do_not_collapse_onto_each_other() {
    let (doc, _) = demo_doc();
    let scene = Scene::from_document(&doc);
    for frame in scene.frames.values() {
        let worlds: Vec<_> = frame
            .children
            .iter()
            .map(|id| scene.frames[id].world)
            .collect();
        for (i, a) in worlds.iter().enumerate() {
            for b in worlds.iter().skip(i + 1) {
                let same = (a.x - b.x).abs() < 1e-9
                    && (a.y - b.y).abs() < 1e-9
                    && (a.w - b.w).abs() < 1e-9
                    && (a.h - b.h).abs() < 1e-9;
                assert!(
                    !same,
                    "siblings in {} share identical world rect {:?}",
                    frame.fqn, a
                );
            }
        }
    }
}

#[test]
fn leaf_payload_includes_source_and_process() {
    let (doc, sources) = demo_doc();
    let node = doc.get(&NodeId::symbol("rust", "demo::left::a")).unwrap();
    let detail = FrameDetail::from_node(node, &sources);
    assert!(detail.source.is_some(), "expected source slice");
    assert!(detail.process.is_some(), "expected process");
    let mermaid = detail.process_mermaid.as_deref().expect("process_mermaid");
    assert!(
        mermaid.starts_with("sequenceDiagram"),
        "expected sequenceDiagram, got {mermaid:?}"
    );
    assert_eq!(detail.source.as_ref().unwrap().text, "fn a() {}");
}

#[test]
fn unresolved_portal_has_no_zoom_target() {
    let (doc, _) = demo_doc();
    let scene = Scene::from_document(&doc);
    let b = scene
        .frames
        .values()
        .find(|f| f.id == "rust://demo::right::b")
        .expect("function b frame");
    let unresolved: Vec<_> = b.portals.iter().filter(|p| p.target.is_none()).collect();
    assert!(
        !unresolved.is_empty(),
        "expected unresolved portal on b, got {:?}",
        b.portals
    );
}

#[test]
fn write_site_emits_scene_and_frames() {
    let (doc, sources) = demo_doc();
    let dir = tempfile_dir();
    let count = write_site(&doc, &sources, &dir).expect("write_site");
    assert!(count >= 2);
    assert!(dir.join("scene.json").is_file());
    assert!(dir.join("scene-entrypoints.json").is_file());
    assert!(dir.join("pipelines.json").is_file());
    assert!(dir.join("frames").is_dir());
    let build_path = dir.join("build.json");
    assert!(build_path.is_file());
    let build: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&build_path).unwrap()).unwrap();
    assert_eq!(build["frames"], count);
    assert!(build["rebuilt_at"].as_u64().is_some());
    let workspace = dir.join(format!("frames/{}.json", stable_id(WORKSPACE_ID)));
    assert!(workspace.is_file());
    // Shared RunRecord index for Runs / Reviews (extract + web from write_site).
    assert!(dir.join("runs/index.json").is_file());
    let index_raw = std::fs::read_to_string(dir.join("runs/index.json")).unwrap();
    assert!(index_raw.contains("\"runs\""));
    let pipelines: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("pipelines.json")).unwrap())
            .unwrap();
    assert!(
        pipelines["recipes"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["id"] == "ci.fmt"),
        "pipelines.json should list CI recipes"
    );
    // No JavaScript in the site data output.
    assert!(!dir.join("viewer.js").exists());
}

#[test]
fn write_site_emits_run_records() {
    let (doc, sources) = demo_doc();
    let dir = tempfile_dir();
    write_site(&doc, &sources, &dir).expect("write_site");
    let index_path = dir.join("runs/index.json");
    assert!(index_path.is_file());
    let index: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&index_path).unwrap()).unwrap();
    let runs = index["runs"].as_array().expect("runs array");
    assert!(runs.len() >= 2, "expected extract + web runs, got {runs:?}");
    let recipes: Vec<_> = runs.iter().filter_map(|r| r["recipe"].as_str()).collect();
    assert!(recipes.contains(&"robby.extract"));
    assert!(recipes.contains(&"robby.web"));
    assert!(dir.join("runs/synthetic-extract.json").is_file());
    assert!(dir.join("runs/latest-web.json").is_file());
    let extract: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(dir.join("runs/synthetic-extract.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(extract["status"], "pass");
    assert_eq!(extract["recipe"], "robby.extract");
    // Shared RunRecord stats are stringly (BTreeMap<String, String>).
    let nodes = extract["stats"]["nodes"].as_str().unwrap_or("");
    assert!(nodes.parse::<u64>().unwrap_or(0) >= 1);
}

#[test]
fn entrypoints_demo_stores_code_paths_in_ir() {
    let doc = entrypoints_demo_doc();
    assert_eq!(doc.code_paths.len(), 2);
    assert!(
        doc.code_paths
            .iter()
            .any(|p| p.entry == NodeId::symbol("rust", "demo::bin_a::main") && !p.edges.is_empty()),
        "bin_a main should have call-tree edges"
    );
}

#[test]
fn frame_detail_includes_execution_mermaid_for_call_path() {
    let doc = entrypoints_demo_doc();
    let sources = BTreeMap::new();
    let node = doc
        .get(&NodeId::symbol("rust", "demo::bin_a::main"))
        .expect("main");
    let detail = FrameDetail::from_node_in_doc(node, &sources, Some(&doc));
    let exec = detail.execution.as_ref().expect("execution");
    assert!(
        !exec.participants.is_empty(),
        "expected execution participants"
    );
    assert!(
        exec.participants.iter().any(|p| {
            p.label.contains("shared")
                || p.label.contains("helper")
                || p.target.is_some()
        }),
        "expected callees or portal targets in execution: {:?}",
        exec.participants
    );
    let mermaid = detail
        .execution_mermaid
        .as_deref()
        .expect("execution_mermaid");
    assert!(
        mermaid.starts_with("sequenceDiagram"),
        "expected sequenceDiagram, got {mermaid:?}"
    );
    assert!(
        mermaid.contains("shared") || mermaid.contains("helper"),
        "expected callees in diagram: {mermaid}"
    );
}

#[test]
fn leaf_process_does_not_duplicate_as_execution() {
    let (doc, sources) = demo_doc();
    let node = doc.get(&NodeId::symbol("rust", "demo::left::a")).unwrap();
    let detail = FrameDetail::from_node_in_doc(node, &sources, Some(&doc));
    assert!(
        detail.process_mermaid.is_some(),
        "definition Process should still render"
    );
    assert!(
        detail.execution_mermaid.is_none(),
        "no Calls / no observations → Execution must stay empty, got {:?}",
        detail.execution_mermaid
    );
}

#[test]
fn partial_observations_yield_distinct_execution() {
    let sources = BTreeMap::new();
    let mut doc = Document::new("test");
    let id = NodeId::symbol("rust", "demo::left::a");
    let mut node = Node::new(id.clone(), code::FUNCTION, "demo::left::a");
    let mut proc = Process::new();
    proc.participant("a", "a", Kind::from(code::FUNCTION));
    proc.participant("helper", "helper", Kind::from(code::FUNCTION));
    proc.step(Step {
        from: "a".to_owned(),
        to: "a".to_owned(),
        label: "body".to_owned(),
        kind: StepKind::Note,
        observed: false,
        span: None,
        blocks: vec![],
    });
    proc.step(Step {
        from: "a".to_owned(),
        to: "helper".to_owned(),
        label: "seen".to_owned(),
        kind: StepKind::Call,
        observed: true,
        span: None,
        blocks: vec![],
    });
    node.process = Some(proc);
    doc.insert(node);

    let node = doc.get(&id).unwrap();
    let detail = FrameDetail::from_node_in_doc(node, &sources, Some(&doc));
    let process = detail.process_mermaid.as_deref().expect("process_mermaid");
    let execution = detail
        .execution_mermaid
        .as_deref()
        .expect("execution_mermaid from observations");
    assert_ne!(
        process, execution,
        "Process and Execution must not share the same Mermaid body"
    );
    assert!(
        execution.contains("seen"),
        "execution should include observed step: {execution}"
    );
}

#[test]
fn entrypoints_scene_roots_mains_and_nests_callees() {
    let doc = entrypoints_demo_doc();
    let scene = Scene::from_entrypoints(&doc);
    let root = scene.frames.get(&scene.root).expect("root");
    assert_eq!(root.kind_tag, "entrypoints");
    assert_eq!(root.children.len(), 2, "two main entries");

    let main_a_sid = stable_id("rust://demo::bin_a::main");
    let helper_sid = stable_id("rust://demo::helper");
    let shared_sid = stable_id("rust://demo::shared");
    let main_b_sid = stable_id("rust://demo::bin_b::main");

    assert!(root.children.contains(&main_a_sid));
    assert!(root.children.contains(&main_b_sid));

    let main_a = scene.frames.get(&main_a_sid).expect("main_a");
    assert!(
        main_a.children.contains(&helper_sid) || main_a.children.contains(&shared_sid),
        "main_a should nest a callee, got {:?}",
        main_a.children
    );

    // Shared is placed once (first parent wins).
    let shared = scene.frames.get(&shared_sid).expect("shared");
    let parents_with_shared: Vec<_> = scene
        .frames
        .values()
        .filter(|f| f.children.contains(&shared_sid))
        .map(|f| f.stable_id.as_str())
        .collect();
    assert_eq!(
        parents_with_shared.len(),
        1,
        "shared should have one tree parent, got {parents_with_shared:?}"
    );

    // Other caller surfaces as a portal, not a second parent.
    let main_b = scene.frames.get(&main_b_sid).expect("main_b");
    assert!(
        !main_b.children.contains(&shared_sid),
        "main_b must not re-parent shared"
    );
    assert!(
        main_b
            .portals
            .iter()
            .any(|p| p.target.as_deref() == Some(shared_sid.as_str())),
        "main_b should portal to shared, got {:?}",
        main_b.portals
    );
    assert!(
        shared
            .portals
            .iter()
            .any(|p| p.target.as_deref() == Some(main_b_sid.as_str())),
        "shared should portal back to main_b"
    );
}

#[test]
fn entrypoints_scene_json_is_deterministic() {
    let doc = entrypoints_demo_doc();
    let a = Scene::from_entrypoints(&doc).to_json();
    let b = Scene::from_entrypoints(&doc).to_json();
    assert_eq!(a, b);
}

fn tempfile_dir() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "robby-web-test-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
