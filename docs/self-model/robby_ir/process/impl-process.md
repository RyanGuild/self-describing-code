# `robby_ir::process::impl Process`

[Self-model](../../README.md) · [process](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/process.rs:118`](../../../../crates/robby-ir/src/process.rs#L118) |
| provenance | extractor · confidence 1.00 |

```rust
impl Process {
    pub fn new() -> Self {
        Self {
            participants: Vec::new(),
            steps: Vec::new(),
        }
    }

    /// Add a participant, returning its index. Duplicate ids are ignored.
    pub fn participant(
        &mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        kind: Kind,
    ) -> usize {
        let id = id.into();
        if let Some(idx) = self.participants.iter().position(|p| p.id == id) {
            return idx;
        }
        self.participants.push(Participant {
            id,
            label: label.into(),
            kind,
        });
        self.participants.len() - 1
    }

    pub fn step(&mut self, step: Step) {
        self.steps.push(step);
    }

    /// Merge an observed process into this (static) one.
    ///
    /// Matching is by `(from, to, label)`. Steps seen at runtime are marked
    /// observed; steps known statically but never seen stay dashed. Observed
    /// steps not present statically are appended, because runtime is the
    /// ground truth for what actually happened.
    pub fn absorb(&mut self, observed: &Process) {
        for p in &observed.participants {
            if !self.participants.iter().any(|q| q.id == p.id) {
                self.participants.push(p.clone());
            }
        }

        let mut matched = vec![false; observed.steps.len()];
        for step in &mut self.steps {
            if let Some(idx) = observed
                .steps
                .iter()
                .position(|o| o.from == step.from && o.to == step.to && o.label == step.label)
            {
                step.observed = true;
                matched[idx] = true;
            }
        }
        for (idx, step) in observed.steps.iter().enumerate() {
            if !matched[idx] {
                self.steps.push(step.clone());
            }
        }
    }
}
```

## References

- calls → [`robby_render::report`](../../robby_render/report/index.md) — push

## Referenced by

- calls ← [`robby_extract::rust::build_process_sig`](../../robby_extract/rust/build-process-sig.md)
- calls ← [`robby_extract::rust::impl ProcBuilder<'_, '_>::emit`](../../robby_extract/rust/index.md)
- calls ← [`robby_extract::typescript::build_function_process`](../../robby_extract/typescript/build-function-process.md)
- calls ← [`robby_extract::typescript::emit_variable`](../../robby_extract/typescript/emit-variable.md)
- calls ← [`robby_extract::typescript::ensure_participant`](../../robby_extract/typescript/ensure-participant.md)
- calls ← [`robby_ir::process::impl Default for Process::default`](index.md)
- calls ← [`robby_render::test_markdown::fixture`](../../robby_render/test_markdown/fixture.md)
- calls ← [`robby_render::test_render::process`](../../robby_render/test_render/process.md)
- calls ← [`robby_render::test_web::demo_doc`](../../robby_render/test_web/demo-doc.md)
- calls ← [`robby_render::view::execution::add_callable`](../../robby_render/view/execution/add-callable.md)
- calls ← [`robby_render::view::execution::from_code_path`](../../robby_render/view/execution/from-code-path.md)

### `new`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn new(0)` |
| location | [`crates/robby-ir/src/process.rs:119`](../../../../crates/robby-ir/src/process.rs#L119) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn new() -> Self {
        Self {
            participants: Vec::new(),
            steps: Vec::new(),
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant new as new()
    participant Vec__new as Vec::new
    new-->>Vec__new: Vec::new()
    new-->>Vec__new: Vec::new()
```

2 steps · 0 observed · 2 possible only

**Edges**

- calls → [`robby_ir::process::impl Process::new`](index.md)
- calls → [`robby_ir::process::impl Process::new`](index.md)

### `participant`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn participant(4)` |
| location | [`crates/robby-ir/src/process.rs:126`](../../../../crates/robby-ir/src/process.rs#L126) |
| provenance | extractor · confidence 1.00 |

> Add a participant, returning its index. Duplicate ids are ignored.

```rust
/// Add a participant, returning its index. Duplicate ids are ignored.
    pub fn participant(
        &mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        kind: Kind,
    ) -> usize {
        let id = id.into();
        if let Some(idx) = self.participants.iter().position(|p| p.id == id) {
            return idx;
        }
        self.participants.push(Participant {
            id,
            label: label.into(),
            kind,
        });
        self.participants.len() - 1
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant participant as participant()
    participant into as into
    participant iter as iter
    participant position as position
    participant push as push
    participant len as len
    participant-->>into: id.into()
    participant-->>iter: self.participants.iter()
    participant-->>position: self.participants.iter().position(|p| p.id == id)
    alt let Some(idx) = self.participants.iter().position(|p| p.id == id)
    participant-->>participant: return
    end
    participant-->>into: label.into()
    participant-->>push: self.participants.push(Participant { id, label: label.into(), kind, })
    participant-->>len: self.participants.len()
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)

### `step`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn step(2)` |
| location | [`crates/robby-ir/src/process.rs:145`](../../../../crates/robby-ir/src/process.rs#L145) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn step(&mut self, step: Step) {
        self.steps.push(step);
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant step as step()
    participant push as push
    step-->>push: self.steps.push(step)
```

1 steps · 0 observed · 1 possible only

**Edges**

- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)

### `absorb`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn absorb(2)` |
| location | [`crates/robby-ir/src/process.rs:149`](../../../../crates/robby-ir/src/process.rs#L149) |
| provenance | extractor · confidence 1.00 |

> Merge an observed process into this (static) one.
> 
> Matching is by `(from, to, label)`. Steps seen at runtime are marked
> observed; steps known statically but never seen stay dashed. Observed
> steps not present statically are appended, because runtime is the
> ground truth for what actually happened.

```rust
/// Merge an observed process into this (static) one.
    ///
    /// Matching is by `(from, to, label)`. Steps seen at runtime are marked
    /// observed; steps known statically but never seen stay dashed. Observed
    /// steps not present statically are appended, because runtime is the
    /// ground truth for what actually happened.
    pub fn absorb(&mut self, observed: &Process) {
        for p in &observed.participants {
            if !self.participants.iter().any(|q| q.id == p.id) {
                self.participants.push(p.clone());
            }
        }

        let mut matched = vec![false; observed.steps.len()];
        for step in &mut self.steps {
            if let Some(idx) = observed
                .steps
                .iter()
                .position(|o| o.from == step.from && o.to == step.to && o.label == step.label)
            {
                step.observed = true;
                matched[idx] = true;
            }
        }
        for (idx, step) in observed.steps.iter().enumerate() {
            if !matched[idx] {
                self.steps.push(step.clone());
            }
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant absorb as absorb()
    participant iter as iter
    participant any as any
    participant clone as clone
    participant push as push
    participant position as position
    participant enumerate as enumerate
    loop &observed.participants
    absorb-->>iter: self.participants.iter()
    absorb-->>any: self.participants.iter().any(|q| q.id == p.id)
    alt !self.participants.iter().any(|q| q.id == p.id)
    absorb-->>clone: p.clone()
    absorb-->>push: self.participants.push(p.clone())
    end
    end
    absorb-->>absorb: vec![false observed.steps.len()]
    loop &mut self.steps
    absorb-->>iter: observed .steps .iter()
    absorb-->>position: observed .steps .iter() .position(|o| o.from == step.from && o.to == ...
    end
    absorb-->>iter: observed.steps.iter()
    absorb-->>enumerate: observed.steps.iter().enumerate()
    loop observed.steps.iter().enumerate()
    alt !matched[idx]
    absorb-->>clone: step.clone()
    absorb-->>push: self.steps.push(step.clone())
    end
    end
```

11 steps · 0 observed · 11 possible only

**Edges**

- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)

