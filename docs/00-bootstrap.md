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

The tool now renders the whole self-model to one file instead of leaving the
diagrams scattered as `.mmd` fragments:

```
$ robby report --ir fixtures/m0/hello.ir.json --out docs/robby-self-model.md
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
  → build.rs loads the IR
  → enforces the coverage invariant
  → writes docs/robby-self-model.md
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
cargo build -p robby-cli                         → docs/robby-self-model.md written
ROBBY_IR=/tmp/broken.ir.json cargo build -p robby-cli
  robby: unclaimed code at fixtures/m0/hello.rs:14:18  "d(path)?; Ok(raw.len()) }"
  robby: coverage invariant violated — 245 bytes claimed, 30 unclaimed
  exit 101
```

### Known gap

`build.rs` and the example are themselves code, and the map does not yet cover
them — the IR is still hand-authored for a fixture. That is exactly what M1
fixes: once `robby-extract` walks the workspace, the tool's own build script
appears in its own map. Until then this is a documented hole, not a silent one.


