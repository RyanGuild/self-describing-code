# Bootstrap log

A running record of what was built, in order, and why. Each step ends with a
verification, so the log doubles as an audit trail for the "total
representation" invariant.

---

## Step 0 — Environment

| Tool | Version |
|---|---|
| rustc | 1.99.0 stable |
| cargo | 1.99.0 |
| node | 26.4.0 |
| npm | 11.17.0 |
| syn | 3.0.9 |
| schemars | 1.2.3 |
| mermaid | 12.2.0 |

`syn` 3 is API-compatible with `syn` 2 for the surface we use (`File`, `Item`,
`Expr`, `DeriveInput`, `parse_file`). No nightly required — the whole pipeline
is stable-only by design.

## Step 1 — The IR contract (`robby-ir`)

Defined before any extractor, on purpose. The schema is the thing both
producers and the view agree on; encoding assumptions into an extractor first
would make the schema a description of that extractor instead of a description
of Robby.

Four invariants are documented in the crate root:

1. Total representation — every byte owned or classified.
2. Diagrams are views — `Process` is stored, Mermaid is derived.
3. Open kinds — namespaced strings, so `agent.*` needs no schema break.
4. Edges are never dropped — unresolved edges keep `resolved: false`.

### Verification

`cargo build -p robby-ir` — clean.

## Step 2 — The coverage gate

The riskiest component, built second so it can fail early. `file_coverage`
unions all node spans, computes the byte complement, and classifies each gap by
scanning for whitespace, comments (`//`, nested `/* */`) and attributes
(`#[...]`, `#![...]`, string-aware).

Only `GapReason::Unknown` fails. Comments and attributes are *classified*, not
ignored — they are raw material for the `why` question, not noise.

### Verification

11 tests, including exact-byte assertions:

```
unclaimed_code_fails_with_the_exact_byte ... ok   (byte 10, line 2, col 1)
nested_spans_are_unioned_not_double_counted ... ok
unreferenced_files_appear_fully_uncovered ... ok
```

The last one matters: a file that exists but is invisible to the map is
reported at `ratio() == 0.0`, not omitted.

## Step 3 — The renderer (`robby-render`)

Structured `Process` → `sequenceDiagram`. Control-flow nesting is a *frame
stack per step*; the renderer diffs consecutive stacks and emits
`alt`/`else`/`opt`/`loop`/`end`. This keeps the IR flat and diff-friendly while
still expressing nesting.

Observed steps draw solid (`->>`), possible-but-unobserved steps draw dashed
(`-->>`). That single rule is the entire static/runtime merge, visually.

## Step 4 — The report artifact (`robby report`)

The tool renders the self-model to Markdown instead of leaving the diagrams
scattered as `.mmd` fragments:

```
$ robby report --ir .robby/robby.ir.json --out docs/self-model
```

Each node section contains, in order: identity and provenance, the exact source
slice, the sequence diagram, and the observed/possible step counts. The source
slice and the diagram sit next to each other on purpose — a segment that
documents its own process should show the process *and* the code that performs
it, without the reader chasing a second file.

### Verification

```
report_is_byte_stable ... ok
report_embeds_source_and_diagram_together ... ok
fences_are_balanced ... ok
```

`fences_are_balanced` is not cosmetic. A generated Markdown file with an odd
number of fences silently swallows the rest of the document when rendered, and
the failure looks like "the tool forgot my nodes" rather than "the output is
malformed".

## Step 5 — One command

The first version of this step was a shell script, `scripts/self-model.sh`. That
was a mistake, and a specific one: **the script was pipeline code that no map
covered.** A repository whose whole premise is "any code written must be
represented" cannot contain an unrepresented second build path sitting next to
the real one.

So generation moved into `build.rs`:

```
cargo build -p robby-cli
  → build.rs extracts the workspace
  → enforces the coverage invariant
  → writes docs/self-model/
```

If coverage fails, the build fails. Robby cannot be compiled from a codebase
that is invisible to its own map.

### The bootstrap problem

`build.rs` cannot regenerate the artifact when the generation code is what is
broken. That is a genuine deadlock, not a hypothetical.

The escape hatch is `crates/robby-cli/examples/self-model.rs`. It calls the same
`robby_render::pipeline` functions, writes the same files, and reports the same
verdict. Being a Rust example rather than a shell script is the point: it
compiles with the workspace, it is type-checked, and it can be represented in
the map.

| Trigger | Role |
|---|---|
| `build.rs` | default; artifact can never drift |
| `examples/self-model.rs` | escape hatch when the build is broken |
| `robby-cli` subcommands | inspection and CI |

All three call one pipeline. There is no shell glue anywhere in the repo.

### Verification

```
cargo build -p robby-cli                         → docs/self-model/ written
```

## Step 6 — The extractor (`robby-extract`)

Until this step the map described a hand-written fixture, not Robby. That was
the documented hole, and closing it is what makes the self-model real.

`robby-extract` walks every crate under `crates/`, parses each `.rs` file with
`syn`, and emits one node per item with a byte-exact span. Module paths become
the node hierarchy; `use` items, impls, fields, variants, macros and test files
are all represented. Function bodies are walked statement by statement to build
a `Process`: calls become steps, `if`/`match` become `alt`, loops become `loop`.

### Three things that had to change

**Coverage claims became a union, not a frontier.** The frontier model (a parent
may not claim a child's bytes) was wrong for item-level extraction: a function
span legitimately covers its own body, and an impl span legitimately covers the
braces around its methods. The frontier rule forced a synthetic node for every
header, which is noise, not fidelity. `document_coverage` now unions all spans;
`frontier_spans` is kept for the stricter mode. Statement-level depth is a
different check and belongs in the extractor's own tests, where the AST is
available.

**The document declares its files.** `Document.files` is the set of sources the
map claims to describe. The coverage gate reads it, so a file that was never
parsed is a failure rather than an absence. `tests/` is included: an unrepresented
test file is still unrepresented code.

**Impl identity includes generic arguments.** The first run reported exactly two
gaps, both the same bug: `impl From<&str> for Kind` and `impl From<String> for
Kind` collapsed to one id, so one span was silently dropped. Impl ids now carry
the rendered trait and self type. The gate caught a real extractor bug on its
first run, which is the point of building it early.

### Verification

```
crates/robby-ir/src/kind.rs      100.0%   2893/2893
crates/robby-ir/src/model.rs    100.0%  14320/14320
crates/robby-extract/src/lib.rs 100.0%  28491/28491
...
ok: every byte is owned or classified
```

Robby now maps itself: 4 crates, 18 files, ~530 nodes, 100% byte coverage. The
build script and the example are in the map, because they are parsed like any
other source file.

## Step 7 — The map becomes a tree of pages

One 14,000-line Markdown file is technically complete and practically unusable.
Total representation does not require total *concatenation*: the map may be
split, linked, and progressively opened without losing a byte.

The report is now a tree that mirrors the code's own hierarchy:

```
docs/self-model/README.md                     global index: coverage + crate tree
docs/self-model/<crate>/index.md              crate overview
docs/self-model/<crate>/<module>/index.md     module overview: diagram + item table
docs/self-model/<crate>/<module>/<item>.md    one item: source, process, edges
```

Module pages carry the `flowchart`; item pages carry the `sequenceDiagram`. Both
link back up, and every edge links to the page of its target. 260 pages, largest
421 lines.

### Verification

The link graph is tested like any other invariant:

```
every_internal_link_resolves ... ok
no_page_is_a_wall_of_text ... ok        (fails above 1000 lines)
every_module_has_an_index_page ... ok
page_names_never_collide_with_index ... ok
```

`page_names_never_collide_with_index` exists because of a real bug: a function
named `index` overwrote its own module page. Reserving the name is cheap;
discovering the collision in a rendered document is not.
