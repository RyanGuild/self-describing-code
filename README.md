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
| `robby-ir` — schema, model, coverage gate | done, 13 tests |
| `robby-render` — Mermaid + linked Markdown tree | done, 17 tests |
| `robby-extract` — `syn` (Rust) + oxc (TypeScript) producers | done |
| `robby-cli` — `schema`, `check`, `render`, `report`, `extract`, `enrich-comments`, `web` | done |
| `robby-agent` — OpenRouter decision + note enrich | done |
| `build.rs` — extracts the workspace, fails on uncovered code | done |
| `examples/self-model.rs` — bootstrap escape hatch | done |
| TS extract (fixture `ts-mini`) | done |
| `robby-resolve` — real name resolution | next |
| `robby-macros` — additive annotations | next |
| `robby-trace` — process recorder | next |
| site (`apps/site` TypeScript Prezi viewer + `robby web`) | done |

The map describes Robby itself: 4 crates, 18 files, ~530 nodes, 100% byte
coverage. The hand-authored fixture is kept only as a test input.

## Run it

```sh
cargo build -p robby-cli
```

`build.rs` extracts the workspace with `syn`, enforces the coverage invariant,
and writes the generated tree under `docs/self-model/`. If any byte of source is
unclaimed, **the build fails** — Robby cannot be compiled from a codebase that is
invisible to its own map.

Start reading at [`docs/self-model/README.md`](docs/self-model/README.md). It
links to one page per crate, one per module, and one per item.

When the build itself is what is broken, the example is the escape hatch:

```sh
cargo run -p robby-cli --example self-model
cargo run -p robby-cli --example self-model -- --stdout
```

It calls the same pipeline. It is a Rust example rather than a shell script on
purpose: an example is compiled, type-checked, and can be represented in the
map. A shell script beside the build would be a second pipeline that the
self-model cannot see.

Individual subcommands:

```sh
cargo run -p robby-cli -- extract --out .robby/robby.ir.json
cargo run -p robby-cli -- check   --ir .robby/robby.ir.json
cargo run -p robby-cli -- render  --ir .robby/robby.ir.json --out .robby/diagrams
cargo run -p robby-cli -- report  --ir .robby/robby.ir.json --out docs/self-model
cargo run -p robby-cli -- schema  --out schema/robby-ir.schema.json
cargo run -p robby-cli -- enrich-comments --ir .robby/robby.ir.json
cargo run -p robby-cli -- web --ir .robby/robby.ir.json --out apps/site/public
```

`enrich-comments` needs `OPENROUTER_API_KEY`. It writes `agent.decision` nodes into the IR (relevance + optional usage/implementation notes). Report/show stay offline and only read that cache.

`web` writes `scene.json` and lazy frame payloads for the TypeScript Prezi viewer in [`apps/site`](apps/site). Serve it with:

```sh
cd apps/site && npm install && npm run dev
```

## The four invariants

1. **Total representation.** Unclaimed code fails CI.
2. **Diagrams are views.** `Process` is stored; Mermaid is derived.
3. **Open kinds.** `code.*`, `runtime.*`, `agent.*` share one schema.
4. **Edges are never dropped.** Unresolved edges survive with confidence scores.

See `docs/00-bootstrap.md` for the build log and
[`docs/self-model/README.md`](docs/self-model/README.md) for the generated map.
