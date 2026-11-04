//! The self-model is a website made of Markdown. A broken link is a dead end
//! in the map, so links are tested like any other invariant.

use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

use robby_render::tree;

fn workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn pages() -> (robby_ir::Document, Vec<tree::Page>) {
    let root = workspace();
    let doc = robby_extract::extract(&root).expect("extract");
    let pages = tree::tree(&doc, None, None, "docs/self-model");
    (doc, pages)
}

#[test]
fn every_internal_link_resolves() {
    let (_, pages) = pages();
    let known: BTreeSet<String> = pages
        .iter()
        .map(|page| page.path.to_string_lossy().replace('\\', "/"))
        .collect();

    let mut broken = Vec::new();
    for page in &pages {
        let dir = page.path.parent().unwrap_or(Path::new(""));
        for link in markdown_links(&page.body) {
            let target = link.split('#').next().unwrap_or("");
            if !std::path::Path::new(target)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
            {
                continue;
            }
            let resolved = normalize(&dir.join(target));
            if !known.contains(&resolved) {
                broken.push(format!("{}: {link} -> {resolved}", page.path.display()));
            }
        }
    }

    assert!(broken.is_empty(), "broken links:\n{}", broken.join("\n"));
}

#[test]
fn no_page_is_a_wall_of_text() {
    let (_, pages) = pages();
    let worst = pages
        .iter()
        .map(|page| (page.body.lines().count(), page.path.clone()))
        .max()
        .expect("pages");

    assert!(
        worst.0 < 1_000,
        "{} is {} lines; split it further",
        worst.1.display(),
        worst.0
    );
}

#[test]
fn every_module_has_an_index_page() {
    let (doc, pages) = pages();
    let indexes = pages
        .iter()
        .filter(|page| page.path.file_name().is_some_and(|name| name == "index.md"))
        .count();

    let modules = doc
        .nodes
        .values()
        .filter(|node| {
            matches!(
                node.kind.as_str(),
                robby_ir::kind::code::MODULE
                    | robby_ir::kind::code::CRATE
                    | robby_ir::kind::code::PACKAGE
            )
        })
        .count();

    assert_eq!(
        indexes, modules,
        "every module, crate, and package should have exactly one index page"
    );
}

#[test]
fn page_names_never_collide_with_index() {
    let (_, pages) = pages();
    let mut seen = BTreeSet::new();
    for page in &pages {
        let path = page.path.to_string_lossy().to_string();
        assert!(seen.insert(path.clone()), "duplicate page {path}");
    }
}

#[test]
fn modules_link_out_and_back() {
    let (_, pages) = pages();
    let by_path: std::collections::BTreeMap<_, _> = pages
        .iter()
        .map(|page| (page.path.to_string_lossy().replace('\\', "/"), page))
        .collect();

    let cli = by_path.get("robby_cli/index.md").expect("robby_cli index");
    assert!(
        cli.body.contains("## References"),
        "crate pages should list outbound references"
    );
    assert!(
        cli.body.contains("robby_render/pipeline/index.md"),
        "robby_cli should link to pipeline docs:\n{}",
        cli.body
    );

    let pipeline = by_path
        .get("robby_render/pipeline/index.md")
        .expect("pipeline index");
    assert!(
        pipeline.body.contains("## Referenced by"),
        "pipeline should list backlinks"
    );
    assert!(
        pipeline.body.contains("robby_cli"),
        "pipeline should link back to robby_cli callers:\n{}",
        pipeline.body
    );
}

fn markdown_links(body: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut fenced = false;

    for line in body.lines() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        let mut rest = line;
        while let Some(start) = rest.find("](") {
            let after = &rest[start + 2..];
            match after.find(')') {
                Some(end) => {
                    links.push(after[..end].to_owned());
                    rest = &after[end + 1..];
                }
                None => break,
            }
        }
    }

    links
}

fn normalize(path: &Path) -> String {
    let mut parts: Vec<String> = Vec::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                parts.pop();
            }
            Component::Normal(part) => parts.push(part.to_string_lossy().to_string()),
            Component::CurDir | Component::Prefix(_) | Component::RootDir => {}
        }
    }
    parts.join("/")
}
