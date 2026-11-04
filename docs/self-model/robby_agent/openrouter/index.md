# `robby_agent::openrouter`

[Self-model](../../README.md) · [robby_agent](../index.md)

| | |
|---|---|
| kind | `code.module` |
| declared at | [`crates/robby-agent/src/lib.rs:9`](../../../../crates/robby-agent/src/lib.rs#L9) |
| provenance | extractor · confidence 1.00 |
| nodes | 21 |

## Types

```mermaid
classDiagram
    class OpenRouterClient {
        +http
        +api_key
        +base_url
        +from_env()
        +new()
    }
    class ChatCompletion {
        +choices
    }
    class Choice {
        +message
    }
    class ChoiceMessage {
        +content
    }
    class ChatClient {
        <<interface>>
    }
    OpenRouterClient ..|> ChatClient : implements
```

## Package diagram

```mermaid
flowchart TB
    n_e78ea7618cf62376["openrouter"]
    n_356bf2c23da68961(["robby_ir"])
    n_9c636b098ca4450e(["robby_render"])
    n_e78ea7618cf62376 --> n_356bf2c23da68961
    n_e78ea7618cf62376 --> n_9c636b098ca4450e
```

## Module coupling

```mermaid
flowchart TB
    subgraph n_e78ea7618cf62376["openrouter"]
        direction TB
        n_c2cf29bdaa0bcb16["from_env"]
        n_cbea6bc844128b44["new"]
    end
    n_c2cf29bdaa0bcb16 --> n_cbea6bc844128b44
```

## Dependency matrix

| | openrouter | robby_ir | robby_render |
|---|---:|---:|---:|
| **openrouter** | · | 2 | 3 |
| **robby_ir** | | · | |
| **robby_render** | | | · |
## References

- calls → [`robby_ir::run`](../../robby_ir/run/index.md) — is_success
- calls → [`robby_ir::span`](../../robby_ir/span/index.md) — is_empty
- calls → [`robby_render::util`](../../robby_render/util/index.md) — text
- calls → [`robby_render::view::hierarchy`](../../robby_render/view/hierarchy/index.md) — build
- calls → [`robby_render::view::select`](../../robby_render/view/select/index.md) — collect

## Referenced by

- calls ← [`robby_cli::enrich_comments`](../../robby_cli/enrich-comments.md)

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`DEFAULT_BASE`](default-base.md) | `code.const` | — | `crates/robby-agent/src/openrouter.rs:9:1` |
| [`OpenRouterClient`](openrouterclient.md) | `code.struct` | — | `crates/robby-agent/src/openrouter.rs:11:1` |
| [`impl OpenRouterClient`](impl-openrouterclient.md) | `code.impl` | — | `crates/robby-agent/src/openrouter.rs:17:1` |
| [`impl ChatClient for OpenRouterClient`](impl-chatclient-for-openrouterclient.md) | `code.impl` | — | `crates/robby-agent/src/openrouter.rs:41:1` |
| [`ChatCompletion`](chatcompletion.md) | `code.struct` | — | `crates/robby-agent/src/openrouter.rs:93:1` |
| [`Choice`](choice.md) | `code.struct` | — | `crates/robby-agent/src/openrouter.rs:98:1` |
| [`ChoiceMessage`](choicemessage.md) | `code.struct` | — | `crates/robby-agent/src/openrouter.rs:103:1` |

## Imports

- `use anyhow::{Context, Result, bail};`
- `use serde::Deserialize;`
- `use serde_json::json;`
- `use crate::model::{ChatClient, ChatMessage, Role};`

