//! oxc walk over TypeScript packages (`package.json` + `.ts`/`.tsx` sources).

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use oxc::allocator::Allocator;
use oxc::ast::ast::{
    Argument, ArrowFunctionBody, Class, ClassElement, Declaration, Expression, Function,
    ImportDeclaration, MethodDefinition, PropertyKey, Statement, TSInterfaceDeclaration,
    TSTypeName, VariableDeclarator,
};
use oxc::ast::{Comment, CommentPosition};
use oxc::parser::Parser;
use oxc::span::{SourceType, Span as OxcSpan};
use robby_ir::{
    Block, BlockKind, Document, Edge, EdgeKind, Kind, Lang, Node, NodeId, Process, Provenance,
    Source, Span, Step, StepKind, kind,
};
use serde_json::Value;

/// Extract TypeScript packages under `root`.
pub(crate) fn extract(root: &Path) -> Result<Document> {
    let mut doc = Document::new("robby-extract");
    let mut pending: Vec<PendingCall> = Vec::new();

    for package in discover_packages(root)? {
        let mut pkg = Node::with_lang(
            NodeId::symbol("typescript", &package.name),
            kind::code::PACKAGE,
            package.name.clone(),
            Lang::TypeScript,
        );
        pkg.provenance = Provenance::new(Source::Extractor);
        doc.merge(pkg);

        for file in &package.files {
            extract_file(&mut doc, &package, file, &mut pending)?;
        }
    }

    resolve_calls(&mut doc, pending);
    doc.rebuild_code_paths();
    Ok(doc)
}

pub(crate) fn source_file_count(root: &Path) -> Result<usize> {
    Ok(discover_packages(root)?.iter().map(|p| p.files.len()).sum())
}

struct Package {
    name: String,
    dir: PathBuf,
    /// Paths relative to the extract root (workspace / fixture root).
    root: PathBuf,
    files: Vec<PathBuf>,
}

fn discover_packages(root: &Path) -> Result<Vec<Package>> {
    let mut out = Vec::new();

    // Fixture / single-package roots: package.json at the extract root.
    if root.join("package.json").is_file() {
        if let Some(pkg) = package_at(root, root)? {
            out.push(pkg);
        }
        out.sort_by(|a, b| a.name.cmp(&b.name));
        return Ok(out);
    }

    // Workspace layout: only `packages/*` and `apps/*` (skip crates/, fixtures, etc.).
    for dir_name in ["packages", "apps"] {
        let base = root.join(dir_name);
        if !base.is_dir() {
            continue;
        }
        let mut kids: Vec<PathBuf> = fs::read_dir(&base)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.is_dir())
            .collect();
        kids.sort();
        for kid in kids {
            let n = kid.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if n == "node_modules" || n == "dist" {
                continue;
            }
            if let Some(pkg) = package_at(root, &kid)? {
                out.push(pkg);
            }
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

fn package_at(extract_root: &Path, dir: &Path) -> Result<Option<Package>> {
    let pkg_json = dir.join("package.json");
    if !pkg_json.is_file() {
        return Ok(None);
    }
    let fallback = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("package");
    let name = read_package_name(&pkg_json)?.unwrap_or_else(|| sanitize_pkg_name(fallback));
    let files = ts_files(dir)?;
    if files.is_empty() {
        return Ok(None);
    }
    Ok(Some(Package {
        name,
        dir: dir.to_path_buf(),
        root: extract_root.to_path_buf(),
        files,
    }))
}

fn read_package_name(path: &Path) -> Result<Option<String>> {
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let value: Value =
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
    Ok(value
        .get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.trim_start_matches('@').replace('/', "__")))
}

fn sanitize_pkg_name(name: &str) -> String {
    name.replace('-', "_")
}

fn ts_files(package_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    // Prefer src/, else package root (excluding node_modules/dist).
    let src = package_dir.join("src");
    if src.is_dir() {
        collect_ts(&src, &mut files)?;
    } else {
        collect_ts(package_dir, &mut files)?;
    }
    files.sort();
    Ok(files)
}

fn collect_ts(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if name == "node_modules" || name == "dist" {
        return Ok(());
    }
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_ts(&path, out)?;
        } else if is_ts_file(&path) {
            out.push(path);
        }
    }
    Ok(())
}

fn is_ts_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| matches!(e, "ts" | "tsx" | "mts" | "cts"))
}

fn module_fqn(package: &Package, file: &Path) -> String {
    let rel = file
        .strip_prefix(&package.dir)
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/");
    let stem = strip_ts_ext(&rel);
    if stem == "src/index" || stem == "index" {
        return package.name.clone();
    }
    format!("{}/{}", package.name, stem)
}

fn strip_ts_ext(path: &str) -> String {
    for ext in [".tsx", ".mts", ".cts", ".ts"] {
        if let Some(stem) = path.strip_suffix(ext) {
            return stem.to_owned();
        }
    }
    path.to_owned()
}

fn extract_file(
    doc: &mut Document,
    package: &Package,
    file: &Path,
    pending: &mut Vec<PendingCall>,
) -> Result<()> {
    let source = fs::read_to_string(file).with_context(|| format!("reading {}", file.display()))?;
    let rel = file
        .strip_prefix(&package.root)
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/");
    doc.declare_file(rel.clone());

    let allocator = Allocator::default();
    let source_type = SourceType::from_path(file).unwrap_or_else(|_| SourceType::ts());
    let parsed = Parser::new(&allocator, &source, source_type).parse();
    if parsed.fatal_error {
        anyhow::bail!("failed to parse TypeScript file {rel}");
    }

    let fqn = module_fqn(package, file);
    let module_id = NodeId::symbol("typescript", &fqn);
    let mut module = Node::with_lang(
        module_id.clone(),
        kind::code::MODULE,
        fqn.clone(),
        Lang::TypeScript,
    );
    module.provenance = Provenance::new(Source::Extractor);
    if fqn != package.name {
        module.parent = Some(NodeId::symbol("typescript", &package.name));
    }
    // Claim the whole file so trivia (imports between items, etc.) can still
    // fail only on true unknown code; item spans refine the frontier later.
    module.span = Some(Span::new(&rel, &source, 0, source.len() as u32));
    doc.merge(module);

    let ctx = Ctx {
        file: &rel,
        source: &source,
        module: fqn,
    };
    emit_comments(doc, &parsed.program.comments, &module_id, &ctx);
    walk_statements(doc, &parsed.program.body, &module_id, &ctx, pending);
    attach_leading_docs(doc, &parsed.program.comments, &module_id, &ctx);
    Ok(())
}

struct Ctx<'a> {
    file: &'a str,
    source: &'a str,
    module: String,
}

struct PendingCall {
    from: NodeId,
    name: String,
    span: Span,
}

fn walk_statements(
    doc: &mut Document,
    stmts: &[Statement<'_>],
    parent: &NodeId,
    ctx: &Ctx<'_>,
    pending: &mut Vec<PendingCall>,
) {
    for stmt in stmts {
        match stmt {
            Statement::ImportDeclaration(imp) => {
                emit_import(doc, imp, parent, ctx);
            }
            Statement::ExportDeclaration(exp) => {
                walk_declaration(doc, &exp.declaration, parent, ctx, pending);
            }
            Statement::ExportDefaultDeclaration(exp) => {
                // Best-effort: only named function/class defaults.
                match &exp.declaration {
                    oxc::ast::ast::ExportDefaultDeclarationKind::FunctionDeclaration(f) => {
                        emit_function(doc, f, parent, ctx, pending);
                    }
                    oxc::ast::ast::ExportDefaultDeclarationKind::ClassDeclaration(c) => {
                        emit_class(doc, c, parent, ctx, pending);
                    }
                    oxc::ast::ast::ExportDefaultDeclarationKind::TSInterfaceDeclaration(i) => {
                        emit_interface(doc, i, parent, ctx);
                    }
                    _ => {}
                }
            }
            Statement::FunctionDeclaration(f) => emit_function(doc, f, parent, ctx, pending),
            Statement::ClassDeclaration(c) => emit_class(doc, c, parent, ctx, pending),
            Statement::TSInterfaceDeclaration(i) => emit_interface(doc, i, parent, ctx),
            Statement::TSTypeAliasDeclaration(t) => {
                let name = t.id.name.as_str();
                let fqn = format!("{}/{}", ctx.module, name);
                let node = item_node(kind::code::TYPE_ALIAS, &fqn, oxc_span(t.span, ctx), parent);
                doc.merge(node);
            }
            Statement::TSEnumDeclaration(e) => {
                let name = e.id.name.as_str();
                let fqn = format!("{}/{}", ctx.module, name);
                let node = item_node(kind::code::ENUM, &fqn, oxc_span(e.span, ctx), parent);
                doc.merge(node);
            }
            Statement::VariableDeclaration(v) => {
                for decl in &v.declarations {
                    emit_variable(doc, decl, parent, ctx, pending);
                }
            }
            _ => {}
        }
    }
}

fn walk_declaration(
    doc: &mut Document,
    decl: &Declaration<'_>,
    parent: &NodeId,
    ctx: &Ctx<'_>,
    pending: &mut Vec<PendingCall>,
) {
    match decl {
        Declaration::FunctionDeclaration(f) => emit_function(doc, f, parent, ctx, pending),
        Declaration::ClassDeclaration(c) => emit_class(doc, c, parent, ctx, pending),
        Declaration::TSInterfaceDeclaration(i) => emit_interface(doc, i, parent, ctx),
        Declaration::TSTypeAliasDeclaration(t) => {
            let name = t.id.name.as_str();
            let fqn = format!("{}/{}", ctx.module, name);
            let node = item_node(kind::code::TYPE_ALIAS, &fqn, oxc_span(t.span, ctx), parent);
            doc.merge(node);
        }
        Declaration::TSEnumDeclaration(e) => {
            let name = e.id.name.as_str();
            let fqn = format!("{}/{}", ctx.module, name);
            let node = item_node(kind::code::ENUM, &fqn, oxc_span(e.span, ctx), parent);
            doc.merge(node);
        }
        Declaration::VariableDeclaration(v) => {
            for d in &v.declarations {
                emit_variable(doc, d, parent, ctx, pending);
            }
        }
        _ => {}
    }
}

fn emit_import(doc: &mut Document, imp: &ImportDeclaration<'_>, parent: &NodeId, ctx: &Ctx<'_>) {
    let source = js_str(&imp.source.value);
    let fqn = format!("{}/import:{}", ctx.module, source);
    let mut node = item_node(kind::code::IMPORT, &fqn, oxc_span(imp.span, ctx), parent);
    // Resolve relative imports to a module FQN under the same package prefix.
    if let Some(target) = resolve_relative_import(&ctx.module, source) {
        let mut edge = Edge::new(NodeId::symbol("typescript", &target), EdgeKind::Imports);
        edge.span.clone_from(&node.span);
        node.edges.push(edge);
    } else if is_bare_specifier(source) {
        let stub_fqn = format!("npm/{}", npm_bare_name(source));
        let stub_id = NodeId::symbol("typescript", &stub_fqn);
        let mut stub = Node::with_lang(
            stub_id.clone(),
            kind::code::PACKAGE,
            stub_fqn.clone(),
            Lang::TypeScript,
        );
        stub.provenance = Provenance::new(Source::Extractor);
        stub.tags.push("external".into());
        stub.annotations.insert("external".into(), "true".into());
        doc.merge(stub);
        let mut edge = Edge::new(stub_id, EdgeKind::Imports);
        edge.span.clone_from(&node.span);
        node.edges.push(edge);
    }
    doc.merge(node);
}

fn is_bare_specifier(spec: &str) -> bool {
    !spec.is_empty()
        && !spec.starts_with("./")
        && !spec.starts_with("../")
        && !spec.starts_with('/')
}

/// `react/jsx-runtime` → `react`; `@scope/pkg/sub` → `@scope/pkg`.
fn npm_bare_name(spec: &str) -> String {
    if let Some(rest) = spec.strip_prefix('@') {
        let mut parts = rest.splitn(3, '/');
        let scope = parts.next().unwrap_or(rest);
        match parts.next() {
            Some(pkg) if !pkg.is_empty() => format!("@{scope}/{pkg}"),
            _ => format!("@{scope}"),
        }
    } else {
        spec.split('/').next().unwrap_or(spec).to_owned()
    }
}

fn emit_comments(doc: &mut Document, comments: &[Comment], parent: &NodeId, ctx: &Ctx<'_>) {
    for (index, comment) in comments.iter().enumerate() {
        let content = comment.content_span();
        let raw = ctx
            .source
            .get(content.start as usize..content.end as usize)
            .unwrap_or("")
            .trim();
        if raw.is_empty() {
            continue;
        }
        let fqn = format!("{}/comment@{}", ctx.module, comment.span.start);
        let mut node = item_node(
            kind::code::COMMENT,
            &fqn,
            oxc_span(comment.span, ctx),
            parent,
        );
        node.name = format!("comment_{index}");
        node.docs = Some(raw.to_owned());
        let kind_tag = if comment.is_line() { "line" } else { "block" };
        node.annotations
            .insert("comment_kind".into(), kind_tag.into());
        doc.merge(node);
    }
}

/// Attach leading comments (JSDoc-style) onto the following item's `docs`.
///
/// Oxc's `attached_to` is the start of the next *token* (often `export`), while
/// our item spans begin at the declaration keyword (`interface` / `class`).
/// Prefer an exact span match, else the nearest following item.
fn attach_leading_docs(
    doc: &mut Document,
    comments: &[Comment],
    module_id: &NodeId,
    ctx: &Ctx<'_>,
) {
    let children: Vec<NodeId> = doc
        .get(module_id)
        .map(|m| m.children.clone())
        .unwrap_or_default();

    for comment in comments {
        if comment.position != CommentPosition::Leading {
            continue;
        }
        let content = comment.content_span();
        let raw = ctx
            .source
            .get(content.start as usize..content.end as usize)
            .unwrap_or("")
            .trim();
        if raw.is_empty() {
            continue;
        }

        let anchor = if comment.attached_to != 0 {
            comment.attached_to
        } else {
            comment.span.end
        };

        let mut best: Option<(u32, NodeId)> = None;
        for child_id in &children {
            let Some(child) = doc.get(child_id) else {
                continue;
            };
            if matches!(
                child.kind.as_str(),
                kind::code::COMMENT | kind::code::IMPORT
            ) {
                continue;
            }
            let Some(span) = &child.span else {
                continue;
            };
            if span.start.byte < anchor {
                continue;
            }
            if !gap_is_attachable(ctx.source, comment.span.end, span.start.byte) {
                continue;
            }
            match &best {
                Some((byte, _)) if span.start.byte >= *byte => {}
                _ => best = Some((span.start.byte, child_id.clone())),
            }
        }

        if let Some((_, id)) = best
            && let Some(child) = doc.nodes.get_mut(&id)
            && child.docs.is_none()
        {
            child.docs = Some(raw.to_owned());
        }
    }
}

/// True when the bytes between a comment and an item are only whitespace or
/// declaration prefixes (`export` / `default` / `async`).
fn gap_is_attachable(source: &str, from: u32, to: u32) -> bool {
    if to < from {
        return false;
    }
    let gap = source.get(from as usize..to as usize).unwrap_or("");
    gap.split_whitespace()
        .all(|tok| matches!(tok, "export" | "default" | "async" | "declare"))
}

fn resolve_relative_import(from_module: &str, spec: &str) -> Option<String> {
    if !(spec.starts_with("./") || spec.starts_with("../")) {
        return None;
    }
    let pkg = from_module.split('/').next()?;
    let from_dir = {
        let path = from_module.strip_prefix(&format!("{pkg}/")).unwrap_or("");
        Path::new(path)
            .parent()
            .map(std::path::Path::to_path_buf)
            .unwrap_or_default()
    };
    let joined = from_dir.join(spec);
    let mut parts = Vec::new();
    for comp in joined.components() {
        match comp {
            std::path::Component::ParentDir => {
                parts.pop();
            }
            std::path::Component::Normal(s) => parts.push(s.to_string_lossy().into_owned()),
            _ => {}
        }
    }
    let rel = parts.join("/");
    let stem = strip_ts_ext(&rel);
    if stem.is_empty() || stem == "index" {
        Some(pkg.to_owned())
    } else {
        Some(format!("{pkg}/{stem}"))
    }
}

fn emit_variable(
    doc: &mut Document,
    decl: &VariableDeclarator<'_>,
    parent: &NodeId,
    ctx: &Ctx<'_>,
    pending: &mut Vec<PendingCall>,
) {
    let bindings = decl.id.get_binding_identifiers();
    let Some(id) = bindings.first() else {
        return;
    };
    let name = id.name.as_str();
    // Arrow / function expressions assigned to const.
    if let Some(Expression::ArrowFunctionExpression(arrow)) = &decl.init {
        let fqn = format!("{}/{}", ctx.module, name);
        let mut node = item_node(kind::code::FUNCTION, &fqn, oxc_span(decl.span, ctx), parent);
        let mut process = Process::new();
        process.participant(
            sanitize(name),
            format!("{name}()"),
            Kind::from(kind::code::FUNCTION),
        );
        let from = NodeId::symbol("typescript", &fqn);
        match &arrow.body {
            ArrowFunctionBody::FunctionBody(body) => {
                walk_stmts(&body.statements, &mut process, &[], pending, &from, ctx);
            }
            other => {
                if let Some(expr) = other.as_expression() {
                    walk_expr(expr, &mut process, &[], pending, &from, ctx);
                }
            }
        }
        node.process = Some(process);
        doc.merge(node);
    } else if let Some(Expression::FunctionExpression(f)) = &decl.init {
        let fqn = format!("{}/{}", ctx.module, name);
        let mut node = item_node(kind::code::FUNCTION, &fqn, oxc_span(decl.span, ctx), parent);
        node.process = Some(build_function_process(f, &fqn, pending, ctx));
        doc.merge(node);
    }
}

fn emit_function(
    doc: &mut Document,
    f: &Function<'_>,
    parent: &NodeId,
    ctx: &Ctx<'_>,
    pending: &mut Vec<PendingCall>,
) {
    let Some(id) = &f.id else {
        return;
    };
    let name = id.name.as_str();
    let fqn = format!("{}/{}", ctx.module, name);
    let mut node = item_node(kind::code::FUNCTION, &fqn, oxc_span(f.span, ctx), parent);
    node.process = Some(build_function_process(f, &fqn, pending, ctx));
    doc.merge(node);
}

fn emit_interface(
    doc: &mut Document,
    iface: &TSInterfaceDeclaration<'_>,
    parent: &NodeId,
    ctx: &Ctx<'_>,
) {
    let name = iface.id.name.as_str();
    let fqn = format!("{}/{}", ctx.module, name);
    let mut node = item_node(
        kind::code::INTERFACE,
        &fqn,
        oxc_span(iface.span, ctx),
        parent,
    );
    for ext in &iface.extends {
        if let Some(n) = type_name(&ext.type_name) {
            let target = format!("{}/{}", ctx.module, n);
            node.edges.push(Edge::new(
                NodeId::symbol("typescript", &target),
                EdgeKind::Extends,
            ));
        }
    }
    doc.merge(node);
}

fn emit_class(
    doc: &mut Document,
    class: &Class<'_>,
    parent: &NodeId,
    ctx: &Ctx<'_>,
    pending: &mut Vec<PendingCall>,
) {
    let Some(id) = &class.id else {
        return;
    };
    let name = id.name.as_str();
    let fqn = format!("{}/{}", ctx.module, name);
    let class_id = NodeId::symbol("typescript", &fqn);
    let mut node = item_node(kind::code::CLASS, &fqn, oxc_span(class.span, ctx), parent);

    if let Some(heritage) = &class.heritage
        && let Some(super_name) = type_name_expr(&heritage.expression)
    {
        node.edges.push(Edge::new(
            NodeId::symbol("typescript", &format!("{}/{}", ctx.module, super_name)),
            EdgeKind::Extends,
        ));
    }
    for impls in &class.implements {
        if let Some(n) = type_name(&impls.expression) {
            node.edges.push(Edge::new(
                NodeId::symbol("typescript", &format!("{}/{}", ctx.module, n)),
                EdgeKind::Implements,
            ));
        }
    }
    doc.merge(node);

    for el in &class.body.body {
        match el {
            ClassElement::MethodDefinition(m) => {
                emit_method(doc, m, &class_id, &fqn, ctx, pending);
            }
            ClassElement::PropertyDefinition(p) => {
                if let Some(name) = property_key_name(&p.key) {
                    let child = format!("{fqn}/{name}");
                    let n = item_node(kind::code::FIELD, &child, oxc_span(p.span, ctx), &class_id);
                    doc.merge(n);
                }
            }
            _ => {}
        }
    }
}

fn emit_method(
    doc: &mut Document,
    method: &MethodDefinition<'_>,
    parent: &NodeId,
    class_fqn: &str,
    ctx: &Ctx<'_>,
    pending: &mut Vec<PendingCall>,
) {
    let Some(name) = property_key_name(&method.key) else {
        return;
    };
    let fqn = format!("{class_fqn}/{name}");
    let mut node = item_node(kind::code::METHOD, &fqn, oxc_span(method.span, ctx), parent);
    node.process = Some(build_function_process(&method.value, &fqn, pending, ctx));
    doc.merge(node);
}

fn property_key_name(key: &PropertyKey<'_>) -> Option<String> {
    match key {
        PropertyKey::StaticIdentifier(id) => Some(id.name.as_str().to_owned()),
        PropertyKey::Identifier(id) => Some(id.name.as_str().to_owned()),
        PropertyKey::StringLiteral(s) => Some(js_str(&s.value).to_owned()),
        _ => None,
    }
}

fn type_name_expr(expr: &Expression<'_>) -> Option<String> {
    match expr {
        Expression::Identifier(id) => Some(id.name.as_str().to_owned()),
        Expression::StaticMemberExpression(m) => {
            let obj = type_name_expr(&m.object)?;
            Some(format!("{obj}.{}", m.property.name.as_str()))
        }
        _ => None,
    }
}

fn type_name(name: &TSTypeName<'_>) -> Option<String> {
    match name {
        TSTypeName::IdentifierReference(id) => Some(id.name.as_str().to_owned()),
        TSTypeName::QualifiedName(q) => {
            let left = type_name(&q.left)?;
            Some(format!("{left}.{}", q.right.name.as_str()))
        }
        TSTypeName::ThisExpression(_) => Some("this".into()),
    }
}

fn build_function_process(
    f: &Function<'_>,
    fqn: &str,
    pending: &mut Vec<PendingCall>,
    ctx: &Ctx<'_>,
) -> Process {
    let name = f.id.as_ref().map_or_else(
        || fqn.rsplit('/').next().unwrap_or(fqn),
        |i| i.name.as_str(),
    );
    let mut process = Process::new();
    process.participant(
        sanitize(name),
        format!("{name}()"),
        Kind::from(kind::code::FUNCTION),
    );
    let from = NodeId::symbol("typescript", fqn);
    if let Some(body) = &f.body {
        walk_stmts(&body.statements, &mut process, &[], pending, &from, ctx);
    }
    process
}

fn frame(kind: BlockKind, label: &str) -> Block {
    Block {
        kind,
        label: label.into(),
        branch: None,
    }
}

fn walk_stmts(
    stmts: &[Statement<'_>],
    process: &mut Process,
    frames: &[Block],
    pending: &mut Vec<PendingCall>,
    from: &NodeId,
    ctx: &Ctx<'_>,
) {
    for stmt in stmts {
        match stmt {
            Statement::IfStatement(iff) => {
                let mut then_frames = frames.to_vec();
                then_frames.push(frame(BlockKind::Alt, "if"));
                walk_stmt(&iff.consequent, process, &then_frames, pending, from, ctx);
                if let Some(alt) = &iff.alternate {
                    let mut else_frames = frames.to_vec();
                    else_frames.push(frame(BlockKind::Alt, "else"));
                    walk_stmt(alt, process, &else_frames, pending, from, ctx);
                }
            }
            Statement::WhileStatement(w) => {
                let mut loop_frames = frames.to_vec();
                loop_frames.push(frame(BlockKind::Loop, "while"));
                walk_stmt(&w.body, process, &loop_frames, pending, from, ctx);
            }
            Statement::DoWhileStatement(w) => {
                let mut loop_frames = frames.to_vec();
                loop_frames.push(frame(BlockKind::Loop, "do"));
                walk_stmt(&w.body, process, &loop_frames, pending, from, ctx);
            }
            Statement::ForStatement(f) => {
                let mut loop_frames = frames.to_vec();
                loop_frames.push(frame(BlockKind::Loop, "for"));
                walk_stmt(&f.body, process, &loop_frames, pending, from, ctx);
            }
            Statement::ForInStatement(f) => {
                let mut loop_frames = frames.to_vec();
                loop_frames.push(frame(BlockKind::Loop, "for-in"));
                walk_stmt(&f.body, process, &loop_frames, pending, from, ctx);
            }
            Statement::ForOfStatement(f) => {
                let mut loop_frames = frames.to_vec();
                loop_frames.push(frame(BlockKind::Loop, "for-of"));
                walk_stmt(&f.body, process, &loop_frames, pending, from, ctx);
            }
            Statement::SwitchStatement(sw) => {
                for case in &sw.cases {
                    let mut case_frames = frames.to_vec();
                    case_frames.push(frame(BlockKind::Alt, "case"));
                    walk_stmts(&case.consequent, process, &case_frames, pending, from, ctx);
                }
            }
            Statement::TryStatement(t) => {
                walk_stmts(&t.block.body, process, frames, pending, from, ctx);
                if let Some(handler) = &t.handler {
                    let mut catch_frames = frames.to_vec();
                    catch_frames.push(frame(BlockKind::Alt, "catch"));
                    walk_stmts(
                        &handler.body.body,
                        process,
                        &catch_frames,
                        pending,
                        from,
                        ctx,
                    );
                }
                if let Some(finalizer) = &t.finalizer {
                    walk_stmts(&finalizer.body, process, frames, pending, from, ctx);
                }
            }
            Statement::ReturnStatement(r) => {
                if let Some(arg) = &r.argument {
                    walk_expr(arg, process, frames, pending, from, ctx);
                }
                push_step(process, frames, StepKind::Return, "return", "caller");
            }
            Statement::ExpressionStatement(e) => {
                walk_expr(&e.expression, process, frames, pending, from, ctx);
            }
            Statement::BlockStatement(b) => {
                walk_stmts(&b.body, process, frames, pending, from, ctx);
            }
            Statement::VariableDeclaration(v) => {
                for d in &v.declarations {
                    if let Some(init) = &d.init {
                        walk_expr(init, process, frames, pending, from, ctx);
                    }
                }
            }
            _ => {}
        }
    }
}

fn walk_stmt(
    stmt: &Statement<'_>,
    process: &mut Process,
    frames: &[Block],
    pending: &mut Vec<PendingCall>,
    from: &NodeId,
    ctx: &Ctx<'_>,
) {
    walk_stmts(
        std::slice::from_ref(stmt),
        process,
        frames,
        pending,
        from,
        ctx,
    );
}

fn walk_expr(
    expr: &Expression<'_>,
    process: &mut Process,
    frames: &[Block],
    pending: &mut Vec<PendingCall>,
    from: &NodeId,
    ctx: &Ctx<'_>,
) {
    match expr {
        Expression::CallExpression(call) => {
            let name = callee_name(&call.callee).unwrap_or_else(|| "call".into());
            ensure_participant(process, &name);
            push_step(process, frames, StepKind::Call, "self", &sanitize(&name));
            pending.push(PendingCall {
                from: from.clone(),
                name: name.clone(),
                span: oxc_span(call.span, ctx),
            });
            for arg in &call.arguments {
                if let Argument::SpreadElement(s) = arg {
                    walk_expr(&s.argument, process, frames, pending, from, ctx);
                } else if let Some(e) = arg.as_expression() {
                    walk_expr(e, process, frames, pending, from, ctx);
                }
            }
        }
        Expression::NewExpression(new) => {
            let name = callee_name(&new.callee).unwrap_or_else(|| "new".into());
            ensure_participant(process, &name);
            push_step(process, frames, StepKind::Call, "self", &sanitize(&name));
            pending.push(PendingCall {
                from: from.clone(),
                name,
                span: oxc_span(new.span, ctx),
            });
            for arg in &new.arguments {
                if let Some(e) = arg.as_expression() {
                    walk_expr(e, process, frames, pending, from, ctx);
                }
            }
        }
        Expression::AwaitExpression(a) => {
            walk_expr(&a.argument, process, frames, pending, from, ctx);
            push_step(process, frames, StepKind::Signal, "self", "await");
        }
        Expression::ConditionalExpression(c) => {
            walk_expr(&c.test, process, frames, pending, from, ctx);
            let mut then_f = frames.to_vec();
            then_f.push(frame(BlockKind::Alt, "then"));
            walk_expr(&c.consequent, process, &then_f, pending, from, ctx);
            let mut else_f = frames.to_vec();
            else_f.push(frame(BlockKind::Alt, "else"));
            walk_expr(&c.alternate, process, &else_f, pending, from, ctx);
        }
        Expression::BinaryExpression(b) => {
            walk_expr(&b.left, process, frames, pending, from, ctx);
            walk_expr(&b.right, process, frames, pending, from, ctx);
        }
        Expression::LogicalExpression(b) => {
            walk_expr(&b.left, process, frames, pending, from, ctx);
            walk_expr(&b.right, process, frames, pending, from, ctx);
        }
        Expression::AssignmentExpression(a) => {
            walk_expr(&a.right, process, frames, pending, from, ctx);
        }
        Expression::SequenceExpression(s) => {
            for e in &s.expressions {
                walk_expr(e, process, frames, pending, from, ctx);
            }
        }
        Expression::StaticMemberExpression(m) => {
            walk_expr(&m.object, process, frames, pending, from, ctx);
        }
        Expression::ComputedMemberExpression(m) => {
            walk_expr(&m.object, process, frames, pending, from, ctx);
            walk_expr(&m.expression, process, frames, pending, from, ctx);
        }
        Expression::ChainExpression(c) => {
            // Treat optional chaining as a note; still walk inner expression if present.
            push_step(process, frames, StepKind::Note, "self", "optional");
            let _ = c;
        }
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_) => {
            push_step(process, frames, StepKind::Note, "self", "fn");
        }
        Expression::JSXElement(_) | Expression::JSXFragment(_) => {
            push_step(process, frames, StepKind::Note, "self", "jsx");
        }
        Expression::ImportExpression(_) => {
            push_step(process, frames, StepKind::Note, "self", "import()");
        }
        _ => {}
    }
}

fn callee_name(expr: &Expression<'_>) -> Option<String> {
    match expr {
        Expression::Identifier(id) => Some(id.name.as_str().to_owned()),
        Expression::StaticMemberExpression(m) => Some(m.property.name.as_str().to_owned()),
        Expression::ChainExpression(c) => match &c.expression {
            oxc::ast::ast::ChainElement::CallExpression(call) => callee_name(&call.callee),
            oxc::ast::ast::ChainElement::StaticMemberExpression(m) => {
                Some(m.property.name.as_str().to_owned())
            }
            _ => None,
        },
        _ => None,
    }
}

fn push_step(process: &mut Process, frames: &[Block], kind: StepKind, from: &str, to: &str) {
    process.step(Step {
        from: from.into(),
        to: to.into(),
        label: String::new(),
        kind,
        observed: false,
        span: None,
        blocks: frames.to_vec(),
    });
}

fn ensure_participant(process: &mut Process, name: &str) {
    process.participant(
        sanitize(name),
        name.to_owned(),
        Kind::from(kind::code::FUNCTION),
    );
}

fn js_str<'a>(value: &oxc::str::JSStr<'a>) -> &'a str {
    value.as_str().unwrap_or("")
}

fn item_node(kind_name: &str, fqn: &str, span: Span, parent: &NodeId) -> Node {
    let mut node = Node::with_lang(
        NodeId::symbol("typescript", fqn),
        kind_name,
        fqn,
        Lang::TypeScript,
    );
    node.span = Some(span);
    node.parent = Some(parent.clone());
    node.provenance = Provenance::new(Source::Extractor);
    node
}

fn oxc_span(span: OxcSpan, ctx: &Ctx<'_>) -> Span {
    Span::new(ctx.file, ctx.source, span.start, span.end)
}

fn sanitize(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        out.push_str("p_empty");
    }
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.insert(0, 'p');
    }
    out
}

fn resolve_calls(doc: &mut Document, pending: Vec<PendingCall>) {
    let mut by_name: BTreeMap<String, Vec<NodeId>> = BTreeMap::new();
    for node in doc.nodes.values() {
        if matches!(
            node.kind.as_str(),
            kind::code::FUNCTION | kind::code::METHOD
        ) {
            by_name
                .entry(node.name.clone())
                .or_default()
                .push(node.id.clone());
        }
    }

    for call in pending {
        let Some(targets) = by_name.get(&call.name) else {
            continue;
        };
        let target = targets[0].clone();
        let confidence = if targets.len() == 1 { 1.0 } else { 0.4 };
        if let Some(node) = doc.nodes.get_mut(&call.from) {
            let mut edge = Edge::new(target, EdgeKind::Calls);
            edge.span = Some(call.span);
            edge.confidence = confidence;
            if !node.edges.contains(&edge) {
                node.edges.push(edge);
            }
        }
    }
}
