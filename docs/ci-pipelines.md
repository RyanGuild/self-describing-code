# CI as Robby pipelines

GitHub Actions checks are the same **Recipe → Run → Product** model as extract,
enrich, and tests (Project plan: `pipeline-agent-decision-types.md`). Repo ground
truth:

| Artifact | Role |
|---|---|
| `schema/pipelines.json` | Checked-in **Recipe** catalog (`agent.pipeline` ids) |
| `apps/site/public/pipelines.json` | Same catalog, emitted by `robby web` / `write_site` |
| `robby_ir::run::{Recipe, RunRecord, recipes::CI_*}` | Types + ids |
| `.github/workflows/ci.yml` | Invoker; step names = recipe ids |

Kinds: recipe nodes are `agent.pipeline`; each Actions invocation (or step)
becomes an `agent.run` / site `runs/<id>.json` **RunRecord** when absorb lands.

---

## Recipes (CI set)

| Recipe id | Workflow | Invoke | Terminal status | Products |
|---|---|---|---|---|
| `ci.rust` | job `rust` | GHA | pass if all stages pass | child stage runs |
| `ci.fmt` | step `ci.fmt` | `cargo fmt --all -- --check` | pass / fail | (gate only) |
| `ci.clippy` | step `ci.clippy` | `cargo clippy … -D warnings` | pass / fail | (gate only) |
| `ci.test` | step `ci.test` | `cargo test --workspace` (skip page-budget) | pass / fail | test stdout |
| `ci.artifacts` | step `ci.artifacts` | assert generated paths untracked + `git diff --exit-code` | pass / fail | enforces artifact commit policy |
| `ci.site` | job `site` / step `ci.site` | `npm run check` (biome + tsc) | pass / fail | (gate only) |

Related non-CI recipes already in the catalog: `robby.extract`, `robby.check`,
`robby.web`, `robby.enrich_comments`, `cargo.test`, `robby.trace` (planned).

`ci.test` is the **workspace** gate; `cargo.test` is the per-subject invoke used
by the live-tests plan. Same `RunRecord` shape; different recipe id + subject.

---

## Example RunRecord (completed `ci.fmt`)

```json
{
  "run_id": "gha-41927257808-ci.fmt",
  "recipe": "ci.fmt",
  "subject": "workspace:",
  "status": "pass",
  "started_at": 1730000000000,
  "finished_at": 1730000002000,
  "exit_code": 0,
  "stdout_tail": null,
  "stats": { "workflow": "ci", "job": "rust" },
  "products": [],
  "process": {
    "participants": [
      { "id": "checkout", "label": "checkout" },
      { "id": "toolchain", "label": "toolchain" },
      { "id": "fmt", "label": "fmt" }
    ],
    "steps": [
      { "from": "checkout", "to": "toolchain", "label": "toolchain", "observed": true },
      { "from": "toolchain", "to": "fmt", "label": "fmt", "observed": true }
    ]
  }
}
```

`ci.rust` may nest stage Process `ci.fmt → ci.clippy → ci.test → ci.artifacts`
(meta pipeline HUD), matching immersive #8.

---

## Workflow wiring

`.github/workflows/ci.yml` comments every job/step with `# recipe: ci.*` and
names steps `ci.fmt`, `ci.clippy`, … so a future GHA → RunRecord absorber can
key off step name without a second map. Toolchain is **`1.99.0`** (required
input on `dtolnay/rust-toolchain`; matches `rust-toolchain.toml`).

Inventory at time of this doc: **one** workflow file (`ci.yml`), two jobs
(`rust`, `site`). No separate lint/security workflows — keep CI lean.

---

## How to refresh the catalog

1. Edit `robby_ir::run::builtin_recipes()` / `recipes::CI_*`.
2. Refresh `schema/pipelines.json` to the output of `pipelines_json()` (the
   unit test `schema_pipelines_json_matches_builtin` fails until they match).
3. `cargo run -p robby-cli -- web --ir .robby/robby.ir.json --out apps/site/public`
   rewrites `public/pipelines.json` from the same function.
