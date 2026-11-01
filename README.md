# Robby

Robby is a self-describing codebase: every system reports its own process, in a
shared vocabulary, and that report is rendered as a navigable map.

The map is not documentation. It is the interpretability substrate. An agent's
summary can lie by omission; a mechanically derived map cannot. So the first
invariant is not "render the important parts" — it is:

> **Total representation, progressive rendering.**

Every byte of source is owned by a node or classified as trivia. Rendering is
what collapses, clusters, and drills down. Representation never does.

## Status

| Piece | State |
|---|---|
| `robby-ir` — schema, model, coverage gate | done, 12 tests |
| `robby-render` — Mermaid + Markdown views | done, 13 tests |
| `robby-cli` — `schema`, `check`, `render`, `report` | done |
| hand-authored fixture | done |
| `robby-extract` — `syn` walk | next |
| `robby-resolve` — name resolution | next |
| `robby-trace` — process recorder | next |
| site | deferred until the map earns one |

## Run it

```sh
./scripts/self-model.sh
```

That checks the coverage invariant, writes the `.mmd` diagrams to `.robby/m0`,
and regenerates `docs/robby-self-model.md` — a single self-contained file with
each segment's source, its sequence diagram, and its provenance.

```sh
cargo run -p robby-cli -- check  --ir fixtures/m0/hello.ir.json
cargo run -p robby-cli -- render --ir fixtures/m0/hello.ir.json --out .robby/m0
cargo run -p robby-cli -- report --ir fixtures/m0/hello.ir.json --out docs/robby-self-model.md
cargo run -p robby-cli -- schema --out schema/robby-ir.schema.json
```

## The four invariants

1. **Total representation.** Unclaimed code fails CI.
2. **Diagrams are views.** `Process` is stored; Mermaid is derived.
3. **Open kinds.** `code.*`, `runtime.*`, `agent.*` share one schema.
4. **Edges are never dropped.** Unresolved edges survive with confidence scores.

See `docs/00-bootstrap.md` for the build log and `docs/robby-self-model.md` for
the generated map.
