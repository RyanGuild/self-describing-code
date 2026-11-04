# `robby_render::render::markdown::impl Renderer for MarkdownRenderer`

[Self-model](../../../README.md) · [render::markdown](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-render/src/render/markdown.rs:42`](../../../../../crates/robby-render/src/render/markdown.rs#L42) |
| provenance | extractor · confidence 1.00 |

```rust
impl Renderer for MarkdownRenderer {
    fn meta(&self, meta: &NodeMeta) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "| | |");
        let _ = writeln!(out, "|---|---|");
        let _ = writeln!(out, "| kind | `{}` |", meta.kind);
        if let Some(sig) = &meta.signature {
            let _ = writeln!(out, "| signature | `{}` |", cell(sig));
        }
        if let Some(location) = &meta.location {
            let _ = writeln!(out, "| location | `{location}` |");
        }
        let _ = writeln!(
            out,
            "| provenance | {} · confidence {:.2} |",
            meta.provenance_source, meta.confidence
        );
        let _ = writeln!(out, "| nodes | {} |", meta.node_count);
        out.push('\n');
        if let Some(docs) = &meta.docs {
            for line in docs.lines() {
                let _ = writeln!(out, "> {line}");
            }
            out.push('\n');
        }
        out
    }

    fn process(&self, view: &ProcessView) -> String {
        Self::fence("mermaid", &self.mermaid.process(view))
    }

    fn types(&self, view: &TypeModel) -> String {
        Self::fence("mermaid", &self.mermaid.types(view))
    }

    fn package_deps(&self, view: &PackageStructure) -> String {
        Self::fence("mermaid", &self.mermaid.package_deps(view))
    }

    fn coupling_deps(&self, view: &CouplingGraph) -> String {
        Self::fence("mermaid", &self.mermaid.coupling_deps(view))
    }

    fn dependency_matrix(&self, view: &PackageStructure) -> String {
        mermaid::dsm_markdown_table(view)
    }

    fn hierarchy(&self, _view: &HierarchyView) -> String {
        String::new()
    }

    fn externals(&self, view: &ExternalsView) -> String {
        let mut out = String::new();
        for entry in &view.entries {
            let importers = entry.importers.join(", ");
            let label = if entry.spec == entry.package {
                format!("`{}`", entry.package)
            } else {
                format!("`{}` (`{}`)", entry.package, entry.spec)
            };
            let _ = writeln!(out, "- {label} — imported by `{importers}`");
        }
        out
    }

    fn comments(&self, view: &CommentsView) -> String {
        use crate::view::apply_highlights;

        let mut out = String::new();
        for entry in &view.entries {
            let loc = entry.location.as_deref().unwrap_or("?");
            if let (Some(note_kind), Some(note)) = (entry.note_kind, entry.note.as_ref()) {
                let _ = writeln!(out, "- `{loc}` — **{}**", note_kind.label());
                for line in note.lines() {
                    let _ = writeln!(out, "  > {line}");
                }
                if let Some(code) = &entry.code {
                    let marked = apply_highlights(code, &entry.highlights);
                    out.push_str("  ```\n");
                    for line in marked.trim_end().lines() {
                        let _ = writeln!(out, "  {line}");
                    }
                    out.push_str("  ```\n");
                }
                out.push('\n');
                continue;
            }
            let kind = entry.kind.as_str();
            let text = trim_comment(&entry.text, 120);
            if let Some(owner) = &entry.owner_fqn {
                let _ = writeln!(out, "- `{loc}` ({kind}, `{owner}`): {text}");
            } else {
                let _ = writeln!(out, "- `{loc}` ({kind}): {text}");
            }
            if let Some(code) = &entry.code {
                let marked = apply_highlights(code, &entry.highlights);
                out.push_str("  ```\n");
                for line in marked.trim_end().lines() {
                    let _ = writeln!(out, "  {line}");
                }
                out.push_str("  ```\n");
            }
        }
        out
    }

    fn links(&self, view: &LinkGraph) -> String {
        let mut out = String::new();
        if !view.outbound.is_empty() {
            out.push_str("## References\n\n");
            for link in &view.outbound {
                let detail = if link.items.len() <= 4 {
                    format!(" — {}", link.items.join(", "))
                } else {
                    format!(" — {} items", link.items.len())
                };
                let _ = writeln!(
                    out,
                    "- {} → `{}`{detail}",
                    link.kind_label, link.module_fqn
                );
            }
            out.push('\n');
        }
        if !view.inbound.is_empty() {
            out.push_str("## Referenced by\n\n");
            for link in &view.inbound {
                let kinds = link.kinds.join(", ");
                let _ = writeln!(out, "- {kinds} ← `{}`", link.caller_fqn);
            }
            out.push('\n');
        }
        out
    }

    fn sibling_graph(&self, view: &SiblingGraph) -> String {
        Self::fence("mermaid", &self.mermaid.sibling_graph(view))
    }

    fn source(&self, view: &SourceSlice) -> String {
        let mut out = String::from("```rust\n");
        out.push_str(view.text.trim_end());
        out.push_str("\n```\n");
        out
    }
}
```

## References

- calls → [`robby_ir::kind`](../../../robby_ir/kind/index.md) — from
- calls → [`robby_render::render::markdown`](index.md) — fence, new, trim_comment
- calls → [`robby_render::render::mermaid`](../mermaid/index.md) — dsm_markdown_table
- calls → [`robby_render::report`](../../report/index.md) — push
- calls → [`robby_render::view::comments`](../../view/comments/index.md) — apply_highlights, as_str
- calls → [`robby_render::view::links`](../../view/links/index.md) — is_empty

### `meta`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn meta(2)` |
| location | [`crates/robby-render/src/render/markdown.rs:43`](../../../../../crates/robby-render/src/render/markdown.rs#L43) |
| provenance | extractor · confidence 1.00 |

```rust
fn meta(&self, meta: &NodeMeta) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "| | |");
        let _ = writeln!(out, "|---|---|");
        let _ = writeln!(out, "| kind | `{}` |", meta.kind);
        if let Some(sig) = &meta.signature {
            let _ = writeln!(out, "| signature | `{}` |", cell(sig));
        }
        if let Some(location) = &meta.location {
            let _ = writeln!(out, "| location | `{location}` |");
        }
        let _ = writeln!(
            out,
            "| provenance | {} · confidence {:.2} |",
            meta.provenance_source, meta.confidence
        );
        let _ = writeln!(out, "| nodes | {} |", meta.node_count);
        out.push('\n');
        if let Some(docs) = &meta.docs {
            for line in docs.lines() {
                let _ = writeln!(out, "> {line}");
            }
            out.push('\n');
        }
        out
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant meta as meta()
    participant String__new as String::new
    participant push as push
    participant lines as lines
    meta-->>String__new: String::new()
    meta-->>meta: writeln!(out, '| | |')
    meta-->>meta: writeln!(out, '|---|---|')
    meta-->>meta: writeln!(out, '| kind | `{}` |', meta.kind)
    alt let Some(sig) = &meta.signature
    meta-->>meta: writeln!(out, '| signature | `{}` |', cell(sig))
    end
    alt let Some(location) = &meta.location
    meta-->>meta: writeln!(out, '| location | `{location}` |')
    end
    meta-->>meta: writeln!( out, '| provenance | {} · confidence {:.2} |', meta.provena...
    meta-->>meta: writeln!(out, '| nodes | {} |', meta.node_count)
    meta-->>push: out.push('\n')
    alt let Some(docs) = &meta.docs
    meta-->>lines: docs.lines()
    loop docs.lines()
    meta-->>meta: writeln!(out, '> {line}')
    end
    meta-->>push: out.push('\n')
    end
```

12 steps · 0 observed · 12 possible only

**Edges**

- calls → [`robby_render::render::markdown::impl MarkdownRenderer::new`](index.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)

### `process`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn process(2)` |
| location | [`crates/robby-render/src/render/markdown.rs:70`](../../../../../crates/robby-render/src/render/markdown.rs#L70) |
| provenance | extractor · confidence 1.00 |

```rust
fn process(&self, view: &ProcessView) -> String {
        Self::fence("mermaid", &self.mermaid.process(view))
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant process as process()
    participant Self__fence as Self::fence
    process-->>process: self.mermaid.process(view)
    process-->>Self__fence: Self::fence('mermaid', &self.mermaid.process(view))
```

2 steps · 0 observed · 2 possible only

**Edges**

- calls → [`robby_render::render::markdown::impl Renderer for MarkdownRenderer::process`](index.md)
- calls → [`robby_render::render::markdown::impl MarkdownRenderer::fence`](index.md)

### `types`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn types(2)` |
| location | [`crates/robby-render/src/render/markdown.rs:74`](../../../../../crates/robby-render/src/render/markdown.rs#L74) |
| provenance | extractor · confidence 1.00 |

```rust
fn types(&self, view: &TypeModel) -> String {
        Self::fence("mermaid", &self.mermaid.types(view))
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant types as types()
    participant Self__fence as Self::fence
    types-->>types: self.mermaid.types(view)
    types-->>Self__fence: Self::fence('mermaid', &self.mermaid.types(view))
```

2 steps · 0 observed · 2 possible only

**Edges**

- calls → [`robby_render::render::markdown::impl Renderer for MarkdownRenderer::types`](index.md)
- calls → [`robby_render::render::markdown::impl MarkdownRenderer::fence`](index.md)

### `package_deps`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn package_deps(2)` |
| location | [`crates/robby-render/src/render/markdown.rs:78`](../../../../../crates/robby-render/src/render/markdown.rs#L78) |
| provenance | extractor · confidence 1.00 |

```rust
fn package_deps(&self, view: &PackageStructure) -> String {
        Self::fence("mermaid", &self.mermaid.package_deps(view))
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant package_deps as package_deps()
    participant Self__fence as Self::fence
    package_deps-->>package_deps: self.mermaid.package_deps(view)
    package_deps-->>Self__fence: Self::fence('mermaid', &self.mermaid.package_deps(view))
```

2 steps · 0 observed · 2 possible only

**Edges**

- calls → [`robby_render::render::markdown::impl Renderer for MarkdownRenderer::package_deps`](index.md)
- calls → [`robby_render::render::markdown::impl MarkdownRenderer::fence`](index.md)

### `coupling_deps`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn coupling_deps(2)` |
| location | [`crates/robby-render/src/render/markdown.rs:82`](../../../../../crates/robby-render/src/render/markdown.rs#L82) |
| provenance | extractor · confidence 1.00 |

```rust
fn coupling_deps(&self, view: &CouplingGraph) -> String {
        Self::fence("mermaid", &self.mermaid.coupling_deps(view))
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant coupling_deps as coupling_deps()
    participant Self__fence as Self::fence
    coupling_deps-->>coupling_deps: self.mermaid.coupling_deps(view)
    coupling_deps-->>Self__fence: Self::fence('mermaid', &self.mermaid.coupling_deps(view))
```

2 steps · 0 observed · 2 possible only

**Edges**

- calls → [`robby_render::render::markdown::impl Renderer for MarkdownRenderer::coupling_deps`](index.md)
- calls → [`robby_render::render::markdown::impl MarkdownRenderer::fence`](index.md)

### `dependency_matrix`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn dependency_matrix(2)` |
| location | [`crates/robby-render/src/render/markdown.rs:86`](../../../../../crates/robby-render/src/render/markdown.rs#L86) |
| provenance | extractor · confidence 1.00 |

```rust
fn dependency_matrix(&self, view: &PackageStructure) -> String {
        mermaid::dsm_markdown_table(view)
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant dependency_matrix as dependency_matrix()
    participant mermaid__dsm_markdown_table as mermaid::dsm_markdown_table
    dependency_matrix-->>mermaid__dsm_markdown_table: mermaid::dsm_markdown_table(view)
```

1 steps · 0 observed · 1 possible only

**Edges**

- calls → [`robby_render::render::mermaid::dsm_markdown_table`](../mermaid/dsm-markdown-table.md)

### `hierarchy`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn hierarchy(2)` |
| location | [`crates/robby-render/src/render/markdown.rs:90`](../../../../../crates/robby-render/src/render/markdown.rs#L90) |
| provenance | extractor · confidence 1.00 |

```rust
fn hierarchy(&self, _view: &HierarchyView) -> String {
        String::new()
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant hierarchy as hierarchy()
    participant String__new as String::new
    hierarchy-->>String__new: String::new()
```

1 steps · 0 observed · 1 possible only

**Edges**

- calls → [`robby_render::render::markdown::impl MarkdownRenderer::new`](index.md)

### `externals`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn externals(2)` |
| location | [`crates/robby-render/src/render/markdown.rs:94`](../../../../../crates/robby-render/src/render/markdown.rs#L94) |
| provenance | extractor · confidence 1.00 |

```rust
fn externals(&self, view: &ExternalsView) -> String {
        let mut out = String::new();
        for entry in &view.entries {
            let importers = entry.importers.join(", ");
            let label = if entry.spec == entry.package {
                format!("`{}`", entry.package)
            } else {
                format!("`{}` (`{}`)", entry.package, entry.spec)
            };
            let _ = writeln!(out, "- {label} — imported by `{importers}`");
        }
        out
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant externals as externals()
    participant String__new as String::new
    participant join as join
    externals-->>String__new: String::new()
    loop &view.entries
    externals-->>join: entry.importers.join(', ')
    alt entry.spec == entry.package
    externals-->>externals: format!('`{}`', entry.package)
    else else
    externals-->>externals: format!('`{}` (`{}`)', entry.package, entry.spec)
    end
    externals-->>externals: writeln!(out, '- {label} — imported by `{importers}`')
    end
```

5 steps · 0 observed · 5 possible only

**Edges**

- calls → [`robby_render::render::markdown::impl MarkdownRenderer::new`](index.md)

### `comments`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn comments(2)` |
| location | [`crates/robby-render/src/render/markdown.rs:108`](../../../../../crates/robby-render/src/render/markdown.rs#L108) |
| provenance | extractor · confidence 1.00 |

```rust
fn comments(&self, view: &CommentsView) -> String {
        use crate::view::apply_highlights;

        let mut out = String::new();
        for entry in &view.entries {
            let loc = entry.location.as_deref().unwrap_or("?");
            if let (Some(note_kind), Some(note)) = (entry.note_kind, entry.note.as_ref()) {
                let _ = writeln!(out, "- `{loc}` — **{}**", note_kind.label());
                for line in note.lines() {
                    let _ = writeln!(out, "  > {line}");
                }
                if let Some(code) = &entry.code {
                    let marked = apply_highlights(code, &entry.highlights);
                    out.push_str("  ```\n");
                    for line in marked.trim_end().lines() {
                        let _ = writeln!(out, "  {line}");
                    }
                    out.push_str("  ```\n");
                }
                out.push('\n');
                continue;
            }
            let kind = entry.kind.as_str();
            let text = trim_comment(&entry.text, 120);
            if let Some(owner) = &entry.owner_fqn {
                let _ = writeln!(out, "- `{loc}` ({kind}, `{owner}`): {text}");
            } else {
                let _ = writeln!(out, "- `{loc}` ({kind}): {text}");
            }
            if let Some(code) = &entry.code {
                let marked = apply_highlights(code, &entry.highlights);
                out.push_str("  ```\n");
                for line in marked.trim_end().lines() {
                    let _ = writeln!(out, "  {line}");
                }
                out.push_str("  ```\n");
            }
        }
        out
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant comments as comments()
    participant String__new as String::new
    participant as_deref as as_deref
    participant unwrap_or as unwrap_or
    participant as_ref as as_ref
    participant lines as lines
    participant apply_highlights as apply_highlights
    participant push_str as push_str
    participant trim_end as trim_end
    participant push as push
    participant as_str as as_str
    participant trim_comment as trim_comment
    comments-->>comments: use crate::view::apply_highlights
    comments-->>String__new: String::new()
    loop &view.entries
    comments-->>as_deref: entry.location.as_deref()
    comments-->>unwrap_or: entry.location.as_deref().unwrap_or('?')
    comments-->>as_ref: entry.note.as_ref()
    alt let (Some(note_kind), Some(note)) = (entry.note_kind, entry.note.as_r...
    comments-->>comments: writeln!(out, '- `{loc}` — **{}**', note_kind.label())
    comments-->>lines: note.lines()
    loop note.lines()
    comments-->>comments: writeln!(out, ' > {line}')
    end
    alt let Some(code) = &entry.code
    comments-->>apply_highlights: apply_highlights(code, &entry.highlights)
    comments-->>push_str: out.push_str(' ```\n')
    comments-->>trim_end: marked.trim_end()
    comments-->>lines: marked.trim_end().lines()
    loop marked.trim_end().lines()
    comments-->>comments: writeln!(out, ' {line}')
    end
    comments-->>push_str: out.push_str(' ```\n')
    end
    comments-->>push: out.push('\n')
    end
    comments-->>as_str: entry.kind.as_str()
    comments-->>trim_comment: trim_comment(&entry.text, 120)
    alt let Some(owner) = &entry.owner_fqn
    comments-->>comments: writeln!(out, '- `{loc}` ({kind}, `{owner}`): {text}')
    else else
    comments-->>comments: writeln!(out, '- `{loc}` ({kind}): {text}')
    end
    alt let Some(code) = &entry.code
    comments-->>apply_highlights: apply_highlights(code, &entry.highlights)
    comments-->>push_str: out.push_str(' ```\n')
    comments-->>trim_end: marked.trim_end()
    comments-->>lines: marked.trim_end().lines()
    loop marked.trim_end().lines()
    comments-->>comments: writeln!(out, ' {line}')
    end
    comments-->>push_str: out.push_str(' ```\n')
    end
    end
```

25 steps · 0 observed · 25 possible only

**Edges**

- calls → [`robby_render::render::markdown::impl MarkdownRenderer::new`](index.md)
- calls → [`robby_render::view::comments::apply_highlights`](../../view/comments/apply-highlights.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)
- calls → [`robby_render::view::comments::impl NoteKind::as_str`](../../view/comments/index.md)
- calls → [`robby_render::render::markdown::trim_comment`](trim-comment.md)
- calls → [`robby_render::view::comments::apply_highlights`](../../view/comments/apply-highlights.md)

### `links`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn links(2)` |
| location | [`crates/robby-render/src/render/markdown.rs:149`](../../../../../crates/robby-render/src/render/markdown.rs#L149) |
| provenance | extractor · confidence 1.00 |

```rust
fn links(&self, view: &LinkGraph) -> String {
        let mut out = String::new();
        if !view.outbound.is_empty() {
            out.push_str("## References\n\n");
            for link in &view.outbound {
                let detail = if link.items.len() <= 4 {
                    format!(" — {}", link.items.join(", "))
                } else {
                    format!(" — {} items", link.items.len())
                };
                let _ = writeln!(
                    out,
                    "- {} → `{}`{detail}",
                    link.kind_label, link.module_fqn
                );
            }
            out.push('\n');
        }
        if !view.inbound.is_empty() {
            out.push_str("## Referenced by\n\n");
            for link in &view.inbound {
                let kinds = link.kinds.join(", ");
                let _ = writeln!(out, "- {kinds} ← `{}`", link.caller_fqn);
            }
            out.push('\n');
        }
        out
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant links as links()
    participant String__new as String::new
    participant is_empty as is_empty
    participant push_str as push_str
    participant len as len
    participant push as push
    participant join as join
    links-->>String__new: String::new()
    links-->>is_empty: view.outbound.is_empty()
    alt !view.outbound.is_empty()
    links-->>push_str: out.push_str(' References\n\n')
    loop &view.outbound
    links-->>len: link.items.len()
    alt link.items.len() <= 4
    links-->>links: format!(' — {}', link.items.join(', '))
    else else
    links-->>links: format!(' — {} items', link.items.len())
    end
    links-->>links: writeln!( out, '- {} → `{}`{detail}', link.kind_label, link.module_fqn )
    end
    links-->>push: out.push('\n')
    end
    links-->>is_empty: view.inbound.is_empty()
    alt !view.inbound.is_empty()
    links-->>push_str: out.push_str(' Referenced by\n\n')
    loop &view.inbound
    links-->>join: link.kinds.join(', ')
    links-->>links: writeln!(out, '- {kinds} ← `{}`', link.caller_fqn)
    end
    links-->>push: out.push('\n')
    end
```

13 steps · 0 observed · 13 possible only

**Edges**

- calls → [`robby_render::render::markdown::impl MarkdownRenderer::new`](index.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../../view/links/index.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../../view/links/index.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)

### `sibling_graph`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn sibling_graph(2)` |
| location | [`crates/robby-render/src/render/markdown.rs:178`](../../../../../crates/robby-render/src/render/markdown.rs#L178) |
| provenance | extractor · confidence 1.00 |

```rust
fn sibling_graph(&self, view: &SiblingGraph) -> String {
        Self::fence("mermaid", &self.mermaid.sibling_graph(view))
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant sibling_graph as sibling_graph()
    participant Self__fence as Self::fence
    sibling_graph-->>sibling_graph: self.mermaid.sibling_graph(view)
    sibling_graph-->>Self__fence: Self::fence('mermaid', &self.mermaid.sibling_graph(view))
```

2 steps · 0 observed · 2 possible only

**Edges**

- calls → [`robby_render::render::markdown::impl Renderer for MarkdownRenderer::sibling_graph`](index.md)
- calls → [`robby_render::render::markdown::impl MarkdownRenderer::fence`](index.md)

### `source`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn source(2)` |
| location | [`crates/robby-render/src/render/markdown.rs:182`](../../../../../crates/robby-render/src/render/markdown.rs#L182) |
| provenance | extractor · confidence 1.00 |

```rust
fn source(&self, view: &SourceSlice) -> String {
        let mut out = String::from("```rust\n");
        out.push_str(view.text.trim_end());
        out.push_str("\n```\n");
        out
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant source as source()
    participant String__from as String::from
    participant trim_end as trim_end
    participant push_str as push_str
    source-->>String__from: String::from('```rust\n')
    source-->>trim_end: view.text.trim_end()
    source-->>push_str: out.push_str(view.text.trim_end())
    source-->>push_str: out.push_str('\n```\n')
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../../robby_ir/kind/index.md)

