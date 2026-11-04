# `robby_render::runs::write_runs`

[Self-model](../../README.md) · [runs](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn write_runs(out: &Path, records: &[RunRecord]) -> Result<()>` |
| location | [`crates/robby-render/src/runs.rs:204`](../../../../crates/robby-render/src/runs.rs#L204) |
| provenance | extractor · confidence 1.00 |

> Write `runs/<id>.json` + `runs/index.json` (newest first).

```rust
/// Write `runs/<id>.json` + `runs/index.json` (newest first).
pub fn write_runs(out: &Path, records: &[RunRecord]) -> Result<()> {
    let runs_dir = out.join("runs");
    fs::create_dir_all(&runs_dir)?;

    let mut index = RunsIndex { runs: Vec::new() };
    for record in records {
        fs::write(
            runs_dir.join(format!("{}.json", record.run_id)),
            record.to_json(),
        )?;
        index.runs.push(record.index_entry());
    }
    // Newest finished_at first for the list.
    index.runs.sort_by(|a, b| b.finished_at.cmp(&a.finished_at));
    fs::write(runs_dir.join("index.json"), index.to_json())?;
    Ok(())
}
```

```mermaid
sequenceDiagram
    autonumber
    participant write_runs as write_runs()
    participant join as join
    participant fs__create_dir_all as fs::create_dir_all
    participant Vec__new as Vec::new
    participant to_json as to_json
    participant fs__write as fs::write
    participant index_entry as index_entry
    participant push as push
    participant cmp as cmp
    participant sort_by as sort_by
    participant Ok as Ok
    write_runs-->>join: out.join('runs')
    write_runs-->>fs__create_dir_all: fs::create_dir_all(&runs_dir)
    write_runs-->>write_runs: ? (propagate)
    write_runs-->>Vec__new: Vec::new()
    loop records
    write_runs-->>write_runs: format!('{}.json', record.run_id)
    write_runs-->>join: runs_dir.join(format!('{}.json', record.run_id))
    write_runs-->>to_json: record.to_json()
    write_runs-->>fs__write: fs::write( runs_dir.join(format!('{}.json', record.run_id)), record.t...
    write_runs-->>write_runs: ? (propagate)
    write_runs-->>index_entry: record.index_entry()
    write_runs-->>push: index.runs.push(record.index_entry())
    end
    write_runs-->>cmp: b.finished_at.cmp(&a.finished_at)
    write_runs-->>sort_by: index.runs.sort_by(|a, b| b.finished_at.cmp(&a.finished_at))
    write_runs-->>join: runs_dir.join('index.json')
    write_runs-->>to_json: index.to_json()
    write_runs-->>fs__write: fs::write(runs_dir.join('index.json'), index.to_json())
    write_runs-->>write_runs: ? (propagate)
    write_runs-->>Ok: Ok(())
```

18 steps · 0 observed · 18 possible only

**Edges**

- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::runs::impl RunRecord::to_json`](index.md)
- calls → [`robby_render::runs::impl RunRecord::index_entry`](index.md)
- calls → [`robby_render::report::impl Report::push`](../report/index.md)
- calls → [`robby_render::runs::impl RunRecord::to_json`](index.md)

## References

- calls → [`robby_render::report`](../report/index.md) — new, push
- calls → [`robby_render::runs`](index.md) — index_entry, to_json

## Referenced by

- calls ← [`robby_render::runs::write_site_runs`](write-site-runs.md)

