//! `syn` walk over a Rust workspace (`<root>/crates`).

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use proc_macro2::LineColumn;
use robby_ir::{
    Block, BlockKind, Document, Edge, EdgeKind, Kind, Lang, Node, NodeId, Process, Provenance,
    Source, Span, Step, StepKind, kind,
};
use syn::spanned::Spanned;
use syn::{Expr, Item, Stmt, Type};

/// Extract a document describing every Rust crate under `<root>/crates`.
pub(crate) fn extract(root: &Path) -> Result<Document> {
    let mut doc = Document::new("robby-extract");
    let mut pending: Vec<PendingCall> = Vec::new();

    let crates = discover_crates(root)?;
    for krate in &crates {
        let mut crate_node = Node::new(
            NodeId::symbol("rust", &krate.name),
            kind::code::CRATE,
            krate.name.clone(),
        );
        crate_node.provenance = Provenance::new(Source::Extractor);
        doc.merge(crate_node);

        for file in &krate.files {
            extract_file(&mut doc, krate, file, &mut pending)?;
        }
    }

    resolve_calls(&mut doc, pending);
    doc.rebuild_code_paths();
    Ok(doc)
}

struct Krate {
    name: String,
    dir: PathBuf,
    files: Vec<PathBuf>,
}

fn discover_crates(root: &Path) -> Result<Vec<Krate>> {
    let crates_dir = root.join("crates");
    let mut out = Vec::new();
    if !crates_dir.is_dir() {
        return Ok(out);
    }

    let mut entries: Vec<PathBuf> = fs::read_dir(&crates_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.join("Cargo.toml").is_file())
        .collect();
    entries.sort();

    for dir in entries {
        let name = dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .replace('-', "_");
        let files = rust_files(&dir)?;
        out.push(Krate { name, dir, files });
    }
    Ok(out)
}

fn rust_files(crate_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_rs(&crate_dir.join("src"), &mut files)?;
    let build = crate_dir.join("build.rs");
    if build.is_file() {
        files.push(build);
    }
    collect_rs(&crate_dir.join("examples"), &mut files)?;
    collect_rs(&crate_dir.join("tests"), &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_rs(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_rs(&path, out)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
    Ok(())
}

fn module_path(krate: &Krate, file: &Path) -> String {
    let rel = file
        .strip_prefix(&krate.dir)
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/");

    if rel == "src/lib.rs" || rel == "src/main.rs" {
        return krate.name.clone();
    }
    if rel == "build.rs" {
        return format!("{}::build", krate.name);
    }
    if let Some(stem) = rel.strip_prefix("examples/").and_then(|s| s.strip_suffix(".rs")) {
        return format!("{}::example_{}", krate.name, stem.replace('-', "_"));
    }
    if let Some(stem) = rel.strip_prefix("tests/").and_then(|s| s.strip_suffix(".rs")) {
        return format!("{}::test_{}", krate.name, stem.replace('-', "_"));
    }
    if let Some(rest) = rel.strip_prefix("src/") {
        let mut p = rest.trim_end_matches(".rs").to_string();
        if p.ends_with("/mod") {
            p.truncate(p.len() - 4);
        }
        return format!("{}::{}", krate.name, p.replace('/', "::"));
    }
    format!("{}::{}", krate.name, rel.trim_end_matches(".rs").replace('/', "::"))
}

fn extract_file(
    doc: &mut Document,
    krate: &Krate,
    file: &Path,
    pending: &mut Vec<PendingCall>,
) -> Result<()> {
    let source = fs::read_to_string(file).with_context(|| format!("reading {}", file.display()))?;
    let rel = file
        .strip_prefix(krate.dir.parent().and_then(|p| p.parent()).unwrap_or(&krate.dir))
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/");

    doc.declare_file(rel.clone());

    let ast = syn::parse_file(&source).with_context(|| format!("parsing {rel}"))?;
    let index = LineIndex::new(&source);
    let path = module_path(krate, file);
    let module_id = NodeId::symbol("rust", &path);

    // File-backed modules hang off their crate so the document has one root per
    // crate. A `mod foo;` declaration may already have set a deeper parent; the
    // merge keeps whatever is there.
    let mut module = Node::new(module_id.clone(), kind::code::MODULE, path.clone());
    module.provenance = Provenance::new(Source::Extractor);
    if path != krate.name {
        module.parent = Some(NodeId::symbol("rust", &krate.name));
    }
    doc.merge(module);

    let ctx = Ctx {
        file: &rel,
        source: &source,
        index: &index,
        module: path.clone(),
    };
    walk_items(doc, &ast.items, &module_id, &ctx, pending);
    Ok(())
}

struct Ctx<'a> {
    file: &'a str,
    source: &'a str,
    index: &'a LineIndex,
    module: String,
}

fn walk_items(
    doc: &mut Document,
    items: &[Item],
    parent: &NodeId,
    ctx: &Ctx<'_>,
    pending: &mut Vec<PendingCall>,
) {
    for item in items {
        let span = span_of(item, ctx);
        let fqn_base = ctx.module.clone();

        match item {
            Item::Fn(f) => {
                let fqn = format!("{fqn_base}::{}", f.sig.ident);
                let mut node = item_node(kind::code::FUNCTION, &fqn, span.clone(), parent);
                node.signature = Some(signature_of(f, ctx));
                node.docs = docs_of(&f.sig, &f.attrs);
                node.process = Some(build_process(f, ctx, &fqn, pending));
                doc.merge(node);
            }
            Item::Struct(s) => {
                let fqn = format!("{fqn_base}::{}", s.ident);
                let mut node = item_node(kind::code::STRUCT, &fqn, span.clone(), parent);
                node.docs = docs_of(&s.ident, &s.attrs);
                for field in &s.fields {
                    if let Some(ident) = &field.ident {
                        let child_fqn = format!("{fqn}::{ident}");
                        let child = item_node(
                            kind::code::FIELD,
                            &child_fqn,
                            span_of(field, ctx),
                            &NodeId::symbol("rust", &fqn),
                        );
                        doc.merge(child);
                        node.children.push(NodeId::symbol("rust", &child_fqn));
                    }
                }
                doc.merge(node);
            }
            Item::Enum(e) => {
                let fqn = format!("{fqn_base}::{}", e.ident);
                let mut node = item_node(kind::code::ENUM, &fqn, span.clone(), parent);
                node.docs = docs_of(&e.ident, &e.attrs);
                for variant in &e.variants {
                    let child_fqn = format!("{fqn}::{}", variant.ident);
                    doc.merge(item_node(
                        kind::code::VARIANT,
                        &child_fqn,
                        span_of(variant, ctx),
                        &NodeId::symbol("rust", &fqn),
                    ));
                    node.children.push(NodeId::symbol("rust", &child_fqn));
                }
                doc.merge(node);
            }
            Item::Union(u) => {
                let fqn = format!("{fqn_base}::{}", u.ident);
                doc.merge(item_node(kind::code::UNION, &fqn, span, parent));
            }
            Item::Trait(t) => {
                let fqn = format!("{fqn_base}::{}", t.ident);
                let mut node = item_node(kind::code::TRAIT, &fqn, span.clone(), parent);
                node.docs = docs_of(&t.ident, &t.attrs);
                for ti in &t.items {
                    if let syn::TraitItem::Fn(m) = ti {
                        let child_fqn = format!("{fqn}::{}", m.sig.ident);
                        doc.merge(item_node(
                            kind::code::METHOD,
                            &child_fqn,
                            span_of(m, ctx),
                            &NodeId::symbol("rust", &fqn),
                        ));
                        node.children.push(NodeId::symbol("rust", &child_fqn));
                    }
                }
                doc.merge(node);
            }
            Item::Impl(i) => {
                let self_name = type_text(&i.self_ty);
                let trait_part = i
                    .trait_
                    .as_ref()
                    .map(|(path, _)| format!("{} for ", path_text(path)))
                    .unwrap_or_default();
                let fqn = format!("{fqn_base}::impl {trait_part}{self_name}");
                let mut node = item_node(kind::code::IMPL, &fqn, span.clone(), parent);
                for ii in &i.items {
                    match ii {
                        syn::ImplItem::Fn(m) => {
                            let child_fqn = format!("{fqn}::{}", m.sig.ident);
                            let mut child = item_node(
                                kind::code::METHOD,
                                &child_fqn,
                                span_of(m, ctx),
                                &NodeId::symbol("rust", &fqn),
                            );
                            child.signature = Some(format!(
                                "fn {}({})",
                                m.sig.ident,
                                m.sig.inputs.len()
                            ));
                            child.docs = docs_of(&m.sig, &m.attrs);
                            child.process =
                                Some(build_process_sig(&m.sig, &m.block, ctx, &child_fqn, pending));
                            doc.merge(child);
                            node.children.push(NodeId::symbol("rust", &child_fqn));
                        }
                        syn::ImplItem::Const(c) => {
                            let child_fqn = format!("{fqn}::{}", c.ident);
                            doc.merge(item_node(
                                kind::code::CONST,
                                &child_fqn,
                                span_of(c, ctx),
                                &NodeId::symbol("rust", &fqn),
                            ));
                            node.children.push(NodeId::symbol("rust", &child_fqn));
                        }
                        syn::ImplItem::Type(t) => {
                            let child_fqn = format!("{fqn}::{}", t.ident);
                            doc.merge(item_node(
                                kind::code::TYPE_ALIAS,
                                &child_fqn,
                                span_of(t, ctx),
                                &NodeId::symbol("rust", &fqn),
                            ));
                            node.children.push(NodeId::symbol("rust", &child_fqn));
                        }
                        _ => {}
                    }
                }
                doc.merge(node);
            }
            Item::Mod(m) => {
                let fqn = format!("{fqn_base}::{}", m.ident);
                let mut node = item_node(kind::code::MODULE, &fqn, span.clone(), parent);
                if let Some((_, items)) = &m.content {
                    node.children.clear();
                    doc.merge(node);
                    walk_items(doc, items, &NodeId::symbol("rust", &fqn), ctx, pending);
                } else {
                    doc.merge(node);
                }
            }
            Item::Const(c) => {
                let fqn = format!("{fqn_base}::{}", c.ident);
                doc.merge(item_node(kind::code::CONST, &fqn, span, parent));
            }
            Item::Static(s) => {
                let fqn = format!("{fqn_base}::{}", s.ident);
                doc.merge(item_node(kind::code::STATIC, &fqn, span, parent));
            }
            Item::Type(t) => {
                let fqn = format!("{fqn_base}::{}", t.ident);
                doc.merge(item_node(kind::code::TYPE_ALIAS, &fqn, span, parent));
            }
            Item::TraitAlias(t) => {
                let fqn = format!("{fqn_base}::{}", t.ident);
                doc.merge(item_node(kind::code::TRAIT_ALIAS, &fqn, span, parent));
            }
            Item::Use(u) => {
                let fqn = format!("{fqn_base}::use@{}", span.start.byte);
                doc.merge(item_node(kind::code::IMPORT, &fqn, span, parent));
                let _ = u;
            }
            Item::ExternCrate(e) => {
                let fqn = format!("{fqn_base}::extern {}", e.ident);
                doc.merge(item_node(kind::code::EXTERN_CRATE, &fqn, span, parent));
            }
            Item::ForeignMod(f) => {
                let fqn = format!("{fqn_base}::extern@{}", span.start.byte);
                doc.merge(item_node(kind::code::FOREIGN_MOD, &fqn, span, parent));
                let _ = f;
            }
            Item::Macro(m) => {
                let name = m
                    .ident
                    .as_ref()
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| "macro".to_owned());
                let fqn = format!("{fqn_base}::{name}@{}", span.start.byte);
                doc.merge(item_node(kind::code::MACRO, &fqn, span, parent));
            }
            _ => {
                let fqn = format!("{fqn_base}::item@{}", span.start.byte);
                doc.merge(item_node(kind::code::VERBATIM, &fqn, span, parent));
            }
        }
    }
}

fn item_node(kind_name: &str, fqn: &str, span: Span, parent: &NodeId) -> Node {
    let mut node = Node::new(NodeId::symbol("rust", fqn), kind_name, fqn);
    node.kind = Kind::from(kind_name);
    node.span = Some(span);
    node.parent = Some(parent.clone());
    node.lang = Lang::Rust;
    node.provenance = Provenance::new(Source::Extractor);
    node
}

fn signature_of(f: &syn::ItemFn, ctx: &Ctx<'_>) -> String {
    let text = slice(ctx, span_of(&f.sig, ctx));
    text.split('{').next().unwrap_or(&text).trim().to_owned()
}

fn docs_of<T: Spanned>(_t: &T, attrs: &[syn::Attribute]) -> Option<String> {
    let mut lines = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) = &nv.value
                {
                    lines.push(s.value().trim().to_owned());
                }
            }
        }
    }
    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}


fn path_text(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|segment| {
            let mut out = segment.ident.to_string();
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                let inner: Vec<String> = args
                    .args
                    .iter()
                    .map(|arg| match arg {
                        syn::GenericArgument::Type(ty) => type_text(ty),
                        syn::GenericArgument::Lifetime(lt) => format!("'{}", lt.ident),
                        _ => "_".to_owned(),
                    })
                    .collect();
                out.push('<');
                out.push_str(&inner.join(", "));
                out.push('>');
            }
            out
        })
        .collect::<Vec<_>>()
        .join("::")
}

/// Render a type just well enough to disambiguate impl blocks.
///
/// `impl From<&str> for Kind` and `impl From<String> for Kind` must not collide,
/// so generic arguments are part of the identity.
fn type_text(ty: &Type) -> String {
    match ty {
        Type::Path(p) => path_text(&p.path),
        Type::Reference(r) => format!(
            "&{}{}",
            if r.mutability.is_some() { "mut " } else { "" },
            type_text(&r.elem)
        ),
        Type::Tuple(t) => format!(
            "({})",
            t.elems.iter().map(type_text).collect::<Vec<_>>().join(", ")
        ),
        Type::Slice(s) => format!("[{}]", type_text(&s.elem)),
        Type::Array(a) => format!("[{}; _]", type_text(&a.elem)),
        Type::Ptr(p) => format!("*{}", type_text(&p.elem)),
        Type::Paren(p) => type_text(&p.elem),
        Type::Group(g) => type_text(&g.elem),
        _ => "_".to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Spans
// ---------------------------------------------------------------------------

/// Converts proc-macro2 line/column positions into byte offsets.
///
/// `proc-macro2`'s `span-locations` reports columns as character offsets within
/// the line, so the conversion walks chars rather than bytes. This is what makes
/// coverage exact on files containing non-ASCII identifiers or comments.
pub struct LineIndex {
    starts: Vec<u32>,
    source: String,
}

impl LineIndex {
    pub fn new(source: &str) -> Self {
        let mut starts = vec![0u32];
        for (i, b) in source.bytes().enumerate() {
            if b == b'\n' {
                starts.push(i as u32 + 1);
            }
        }
        Self {
            starts,
            source: source.to_owned(),
        }
    }

    pub fn byte(&self, lc: LineColumn) -> u32 {
        let line = lc.line.saturating_sub(1).min(self.starts.len() - 1);
        let start = self.starts[line] as usize;
        let rest = &self.source[start..];
        let end = rest.find('\n').unwrap_or(rest.len());
        let line_text = &rest[..end];
        let mut byte = start;
        for (i, ch) in line_text.chars().enumerate() {
            if i >= lc.column {
                break;
            }
            byte += ch.len_utf8();
        }
        byte as u32
    }
}

fn span_of<T: Spanned>(node: &T, ctx: &Ctx<'_>) -> Span {
    let span = node.span();
    let start = ctx.index.byte(span.start());
    let end = ctx.index.byte(span.end());
    Span::new(ctx.file, ctx.source, start, end.max(start))
}

fn slice(ctx: &Ctx<'_>, span: Span) -> String {
    ctx.source
        .get(span.start.byte as usize..span.end.byte as usize)
        .unwrap_or("")
        .to_owned()
}

fn label_of(ctx: &Ctx<'_>, span: Span) -> String {
    let raw = slice(ctx, span);
    let collapsed = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.chars().count() > 72 {
        let cut: String = collapsed.chars().take(69).collect();
        format!("{cut}...")
    } else {
        collapsed
    }
}

// ---------------------------------------------------------------------------
// Process
// ---------------------------------------------------------------------------

struct PendingCall {
    from: NodeId,
    name: String,
    span: Span,
}

fn build_process(
    f: &syn::ItemFn,
    ctx: &Ctx<'_>,
    fqn: &str,
    pending: &mut Vec<PendingCall>,
) -> Process {
    build_process_sig(&f.sig, &f.block, ctx, fqn, pending)
}

fn build_process_sig(
    sig: &syn::Signature,
    block: &syn::Block,
    ctx: &Ctx<'_>,
    fqn: &str,
    pending: &mut Vec<PendingCall>,
) -> Process {
    let me = fqn.rsplit("::").next().unwrap_or(fqn).to_owned();
    let mut builder = ProcBuilder {
        ctx,
        process: Process::new(),
        blocks: Vec::new(),
        me: me.clone(),
        me_id: fqn.to_owned(),
        pending,
    };
    builder.process.participant(
        sanitize(&me),
        format!("{}()", me),
        Kind::from(kind::code::FUNCTION),
    );
    let _ = sig;
    walk_stmts(&mut builder, &block.stmts);
    builder.process
}

struct ProcBuilder<'a, 'b> {
    ctx: &'a Ctx<'b>,
    process: Process,
    blocks: Vec<Block>,
    me: String,
    me_id: String,
    pending: &'a mut Vec<PendingCall>,
}

impl ProcBuilder<'_, '_> {
    fn emit(&mut self, to: &str, label: String, step_kind: StepKind, span: Span) {
        let from = sanitize(&self.me);
        let to_id = sanitize(to);
        if from == to_id {
            self.process
                .participant(to_id.clone(), self.me.clone(), Kind::from(kind::code::FUNCTION));
        } else {
            self.process
                .participant(to_id.clone(), to.to_owned(), Kind::from(kind::code::FUNCTION));
        }
        self.process.step(Step {
            from,
            to: to_id,
            label,
            kind: step_kind,
            observed: false,
            span: Some(span),
            blocks: self.blocks.clone(),
        });
    }

    fn push_block(&mut self, block_kind: BlockKind, label: String) {
        self.blocks.push(Block {
            kind: block_kind,
            label,
            branch: None,
        });
    }

    fn branch(&mut self, name: String) {
        if let Some(last) = self.blocks.last_mut() {
            last.branch = Some(name);
        }
    }

    fn pop_block(&mut self) {
        self.blocks.pop();
    }
}

fn walk_stmts(builder: &mut ProcBuilder<'_, '_>, stmts: &[Stmt]) {
    for stmt in stmts {
        match stmt {
            Stmt::Local(local) => match &local.init {
                Some(init) => walk_expr(builder, &init.expr),
                None => {
                    let span = span_of(local, builder.ctx);
                    builder.emit(
                        &builder.me.clone(),
                        label_of(builder.ctx, span.clone()),
                        StepKind::Note,
                        span,
                    );
                }
            },
            Stmt::Expr(expr, _) => walk_expr(builder, expr),
            Stmt::Macro(m) => {
                let span = span_of(m, builder.ctx);
                let me = builder.me.clone();
                builder.emit(&me, label_of(builder.ctx, span.clone()), StepKind::Note, span);
            }
            Stmt::Item(item) => {
                let span = span_of(item, builder.ctx);
                let me = builder.me.clone();
                builder.emit(&me, label_of(builder.ctx, span.clone()), StepKind::Note, span);
            }
        }
    }
}

fn walk_expr(builder: &mut ProcBuilder<'_, '_>, expr: &Expr) {
    match expr {
        Expr::Call(call) => {
            let span = span_of(expr, builder.ctx);
            let display = callee_display(&call.func);
            let key = callee_name(&call.func);
            let label = label_of(builder.ctx, span.clone());
            // Walk the callee expression (e.g. `(get_fn())()` ) and args for nested calls.
            walk_expr(builder, &call.func);
            for arg in &call.args {
                walk_expr(builder, arg);
            }
            builder.emit(&display, label, StepKind::Call, span.clone());
            builder.pending.push(PendingCall {
                from: NodeId::symbol("rust", &builder.me_id),
                name: key,
                span,
            });
        }
        Expr::MethodCall(m) => {
            let span = span_of(expr, builder.ctx);
            let name = m.method.to_string();
            let label = label_of(builder.ctx, span.clone());
            walk_expr(builder, &m.receiver);
            for arg in &m.args {
                walk_expr(builder, arg);
            }
            builder.emit(&name, label, StepKind::Call, span.clone());
            builder.pending.push(PendingCall {
                from: NodeId::symbol("rust", &builder.me_id),
                name,
                span,
            });
        }
        Expr::If(if_expr) => {
            walk_expr(builder, &if_expr.cond);
            let cond = label_of(builder.ctx, span_of(&if_expr.cond, builder.ctx));
            builder.push_block(BlockKind::Alt, cond);
            walk_stmts(builder, &if_expr.then_branch.stmts);
            if let Some((_, else_expr)) = &if_expr.else_branch {
                builder.branch("else".to_owned());
                walk_expr(builder, else_expr);
            }
            builder.pop_block();
        }
        Expr::ForLoop(f) => {
            walk_expr(builder, &f.expr);
            let label = label_of(builder.ctx, span_of(&f.expr, builder.ctx));
            builder.push_block(BlockKind::Loop, label);
            walk_stmts(builder, &f.body.stmts);
            builder.pop_block();
        }
        Expr::While(w) => {
            walk_expr(builder, &w.cond);
            let label = label_of(builder.ctx, span_of(&w.cond, builder.ctx));
            builder.push_block(BlockKind::Loop, label);
            walk_stmts(builder, &w.body.stmts);
            builder.pop_block();
        }
        Expr::Loop(l) => {
            builder.push_block(BlockKind::Loop, "loop".to_owned());
            walk_stmts(builder, &l.body.stmts);
            builder.pop_block();
        }
        Expr::Match(m) => {
            // Scrutinee holds real calls (`match run() { ... }`) — walk it.
            walk_expr(builder, &m.expr);
            let label = label_of(builder.ctx, span_of(&m.expr, builder.ctx));
            builder.push_block(BlockKind::Alt, format!("match {label}"));
            for (i, arm) in m.arms.iter().enumerate() {
                if i > 0 {
                    let pat = label_of(builder.ctx, span_of(&arm.pat, builder.ctx));
                    builder.branch(pat);
                }
                walk_expr(builder, &arm.body);
            }
            builder.pop_block();
        }
        Expr::Block(b) => walk_stmts(builder, &b.block.stmts),
        Expr::Try(t) => {
            walk_expr(builder, &t.expr);
            let span = span_of(expr, builder.ctx);
            let me = builder.me.clone();
            builder.emit(&me, "? (propagate)".to_owned(), StepKind::Return, span);
        }
        Expr::Return(r) => {
            if let Some(value) = &r.expr {
                walk_expr(builder, value);
            }
            let span = span_of(expr, builder.ctx);
            let me = builder.me.clone();
            builder.emit(&me, "return".to_owned(), StepKind::Return, span);
        }
        Expr::Await(a) => {
            walk_expr(builder, &a.base);
            let span = span_of(expr, builder.ctx);
            let me = builder.me.clone();
            builder.emit(&me, "await".to_owned(), StepKind::Signal, span);
        }
        // Nested call sites inside compound expressions.
        Expr::Paren(p) => walk_expr(builder, &p.expr),
        Expr::Group(g) => walk_expr(builder, &g.expr),
        Expr::Unary(u) => walk_expr(builder, &u.expr),
        Expr::Binary(b) => {
            walk_expr(builder, &b.left);
            walk_expr(builder, &b.right);
        }
        Expr::Assign(a) => {
            walk_expr(builder, &a.left);
            walk_expr(builder, &a.right);
        }
        Expr::Field(f) => walk_expr(builder, &f.base),
        Expr::Index(i) => {
            walk_expr(builder, &i.expr);
            walk_expr(builder, &i.index);
        }
        Expr::Reference(r) => walk_expr(builder, &r.expr),
        Expr::Tuple(t) => {
            for elem in &t.elems {
                walk_expr(builder, elem);
            }
        }
        Expr::Array(a) => {
            for elem in &a.elems {
                walk_expr(builder, elem);
            }
        }
        Expr::Repeat(r) => {
            walk_expr(builder, &r.expr);
            walk_expr(builder, &r.len);
        }
        Expr::Range(r) => {
            if let Some(start) = &r.start {
                walk_expr(builder, start);
            }
            if let Some(end) = &r.end {
                walk_expr(builder, end);
            }
        }
        Expr::Cast(c) => walk_expr(builder, &c.expr),
        Expr::Let(l) => walk_expr(builder, &l.expr),
        Expr::Closure(c) => match &*c.body {
            Expr::Block(b) => walk_stmts(builder, &b.block.stmts),
            other => walk_expr(builder, other),
        },
        Expr::Async(a) => walk_stmts(builder, &a.block.stmts),
        Expr::Unsafe(u) => walk_stmts(builder, &u.block.stmts),
        Expr::Const(c) => walk_stmts(builder, &c.block.stmts),
        Expr::Struct(s) => {
            for field in &s.fields {
                walk_expr(builder, &field.expr);
            }
            if let Some(rest) = &s.rest {
                walk_expr(builder, rest);
            }
        }
        // Leaves — no process step; callers that care emit their own Call/Note.
        Expr::Path(_) | Expr::Lit(_) | Expr::Continue(_) | Expr::Break(_) => {}
        _ => {
            let span = span_of(expr, builder.ctx);
            let me = builder.me.clone();
            builder.emit(&me, label_of(builder.ctx, span.clone()), StepKind::Note, span);
        }
    }
}

/// The readable callee, including its module path: `Document::new`.
fn callee_display(func: &Expr) -> String {
    match func {
        Expr::Path(p) => path_text(&p.path),
        _ => callee_name(func),
    }
}

/// The last segment, used to match against known items.
fn callee_name(func: &Expr) -> String {
    match func {
        Expr::Path(p) => p
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_else(|| "call".to_owned()),
        Expr::Field(f) => match &f.member {
            syn::Member::Named(n) => n.to_string(),
            syn::Member::Unnamed(i) => format!("field{}", i.index),
        },
        _ => "call".to_owned(),
    }
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

// ---------------------------------------------------------------------------
// Call resolution
// ---------------------------------------------------------------------------

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
    for targets in by_name.values_mut() {
        targets.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    }

    for call in pending {
        let Some(targets) = by_name.get(&call.name) else {
            continue;
        };
        let caller_fqn = doc
            .get(&call.from)
            .map(|n| n.fqn.as_str())
            .unwrap_or("");
        let (target, confidence) = pick_call_target(caller_fqn, targets);
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

/// Prefer the callee in the same module as the caller; then longest shared
/// module prefix; then shortest FQN (sibling over nested crate paths).
fn pick_call_target(caller_fqn: &str, targets: &[NodeId]) -> (NodeId, f32) {
    if targets.len() == 1 {
        return (targets[0].clone(), 1.0);
    }
    let caller_mod = module_prefix(caller_fqn);
    let mut ranked: Vec<(i32, i32, usize, &NodeId)> = targets
        .iter()
        .map(|target| {
            let target_fqn = id_fqn(target);
            let target_mod = module_prefix(target_fqn);
            let exact = if target_mod == caller_mod { 1 } else { 0 };
            let shared = shared_module_score(caller_mod, target_mod) as i32;
            // Shorter FQN wins ties (crate::run over crate::nested::run).
            let len = target_fqn.len();
            (-exact, -shared, len, target)
        })
        .collect();
    ranked.sort();
    let best = ranked[0].3;
    let exact = ranked[0].0 == -1;
    let shared = -ranked[0].1;
    let confidence = if exact {
        0.95
    } else if shared > 0 {
        0.7
    } else {
        0.4
    };
    (best.clone(), confidence)
}

fn id_fqn(id: &NodeId) -> &str {
    id.as_str()
        .split_once("://")
        .map(|(_, fqn)| fqn)
        .unwrap_or(id.as_str())
}

fn module_prefix(fqn: &str) -> &str {
    fqn.rsplit_once("::").map(|(m, _)| m).unwrap_or(fqn)
}

fn shared_module_score(a: &str, b: &str) -> usize {
    let a_parts: Vec<&str> = a.split("::").collect();
    let b_parts: Vec<&str> = b.split("::").collect();
    a_parts
        .iter()
        .zip(b_parts.iter())
        .take_while(|(x, y)| x == y)
        .count()
}

/// Count of `.rs` files the extractor considers part of the workspace.
pub(crate) fn source_file_count(root: &Path) -> Result<usize> {
    Ok(discover_crates(root)?.iter().map(|k| k.files.len()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_call_prefers_same_module() {
        let caller = "robby_cli::main";
        let targets = vec![
            NodeId::symbol("rust", "robby_cli::example_self_model::run"),
            NodeId::symbol("rust", "robby_cli::run"),
        ];
        let (picked, conf) = pick_call_target(caller, &targets);
        assert_eq!(picked, NodeId::symbol("rust", "robby_cli::run"));
        assert!(conf >= 0.9);
    }

    #[test]
    fn line_index_handles_non_ascii() {
        let source = "let é = 1;\nfn a() {}";
        let index = LineIndex::new(source);

        // Start of line 2.
        let line_two = index.byte(LineColumn {
            line: 2,
            column: 0,
        });
        assert_eq!(line_two, source.find("fn a").unwrap() as u32);

        // Column 5 is just past the two-byte `é`, so the byte offset is 6.
        let after_e_acute = index.byte(LineColumn {
            line: 1,
            column: 5,
        });
        assert_eq!(after_e_acute, 6);
    }
}
