# `robby_agent::openrouter::impl ChatClient for OpenRouterClient`

[Self-model](../../README.md) · [openrouter](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-agent/src/openrouter.rs:41`](../../../../crates/robby-agent/src/openrouter.rs#L41) |
| provenance | extractor · confidence 1.00 |

```rust
impl ChatClient for OpenRouterClient {
    fn chat(&self, model: &str, messages: &[ChatMessage], json_mode: bool) -> Result<String> {
        let msgs: Vec<_> = messages
            .iter()
            .map(|m| {
                json!({
                    "role": match m.role {
                        Role::System => "system",
                        Role::User => "user",
                        Role::Assistant => "assistant",
                    },
                    "content": m.content,
                })
            })
            .collect();

        let mut body = json!({
            "model": model,
            "messages": msgs,
        });
        if json_mode {
            body["response_format"] = json!({ "type": "json_object" });
        }

        let url = format!("{}/chat/completions", self.base_url);
        let response = self
            .http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://github.com/robby/robby")
            .header("X-Title", "robby")
            .json(&body)
            .send()
            .with_context(|| format!("POST {url}"))?;

        let status = response.status();
        let text = response.text().context("reading OpenRouter body")?;
        if !status.is_success() {
            bail!("OpenRouter {status}: {text}");
        }

        let parsed: ChatCompletion = serde_json::from_str(&text)
            .with_context(|| format!("decoding OpenRouter JSON: {text}"))?;
        parsed
            .choices
            .into_iter()
            .next()
            .and_then(|c| c.message.content)
            .context("OpenRouter response missing choices[0].message.content")
    }
}
```

## References

- calls → [`robby_render::util`](../../robby_render/util/index.md) — text
- calls → [`robby_render::view::select`](../../robby_render/view/select/index.md) — collect

### `chat`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn chat(4)` |
| location | [`crates/robby-agent/src/openrouter.rs:42`](../../../../crates/robby-agent/src/openrouter.rs#L42) |
| provenance | extractor · confidence 1.00 |

```rust
fn chat(&self, model: &str, messages: &[ChatMessage], json_mode: bool) -> Result<String> {
        let msgs: Vec<_> = messages
            .iter()
            .map(|m| {
                json!({
                    "role": match m.role {
                        Role::System => "system",
                        Role::User => "user",
                        Role::Assistant => "assistant",
                    },
                    "content": m.content,
                })
            })
            .collect();

        let mut body = json!({
            "model": model,
            "messages": msgs,
        });
        if json_mode {
            body["response_format"] = json!({ "type": "json_object" });
        }

        let url = format!("{}/chat/completions", self.base_url);
        let response = self
            .http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://github.com/robby/robby")
            .header("X-Title", "robby")
            .json(&body)
            .send()
            .with_context(|| format!("POST {url}"))?;

        let status = response.status();
        let text = response.text().context("reading OpenRouter body")?;
        if !status.is_success() {
            bail!("OpenRouter {status}: {text}");
        }

        let parsed: ChatCompletion = serde_json::from_str(&text)
            .with_context(|| format!("decoding OpenRouter JSON: {text}"))?;
        parsed
            .choices
            .into_iter()
            .next()
            .and_then(|c| c.message.content)
            .context("OpenRouter response missing choices[0].message.content")
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant chat as chat()
    participant iter as iter
    participant map as map
    participant collect as collect
    participant post as post
    participant header as header
    participant json as json
    participant send as send
    participant with_context as with_context
    participant status as status
    participant text as text
    participant context as context
    participant is_success as is_success
    participant serde_json__from_str as serde_json::from_str
    participant into_iter as into_iter
    participant next as next
    participant and_then as and_then
    chat-->>iter: messages .iter()
    chat-->>chat: json!({ 'role': match m.role { Role::System => 'system', Role::User =...
    chat-->>map: messages .iter() .map(|m| { json!({ 'role': match m.role { Role::Syst...
    chat-->>collect: messages .iter() .map(|m| { json!({ 'role': match m.role { Role::Syst...
    chat-->>chat: json!({ 'model': model, 'messages': msgs, })
    alt json_mode
    chat-->>chat: json!({ 'type': 'json_object' })
    end
    chat-->>chat: format!('{}/chat/completions', self.base_url)
    chat-->>post: self .http .post(&url)
    chat-->>chat: format!('Bearer {}', self.api_key)
    chat-->>header: self .http .post(&url) .header('Authorization', format!('Bearer {}', ...
    chat-->>header: self .http .post(&url) .header('Authorization', format!('Bearer {}', ...
    chat-->>header: self .http .post(&url) .header('Authorization', format!('Bearer {}', ...
    chat-->>json: self .http .post(&url) .header('Authorization', format!('Bearer {}', ...
    chat-->>send: self .http .post(&url) .header('Authorization', format!('Bearer {}', ...
    chat-->>chat: format!('POST {url}')
    chat-->>with_context: self .http .post(&url) .header('Authorization', format!('Bearer {}', ...
    chat-->>chat: ? (propagate)
    chat-->>status: response.status()
    chat-->>text: response.text()
    chat-->>context: response.text().context('reading OpenRouter body')
    chat-->>chat: ? (propagate)
    chat-->>is_success: status.is_success()
    alt !status.is_success()
    chat-->>chat: bail!('OpenRouter {status}: {text}')
    end
    chat-->>serde_json__from_str: serde_json::from_str(&text)
    chat-->>chat: format!('decoding OpenRouter JSON: {text}')
    chat-->>with_context: serde_json::from_str(&text) .with_context(|| format!('decoding OpenRo...
    chat-->>chat: ? (propagate)
    chat-->>into_iter: parsed .choices .into_iter()
    chat-->>next: parsed .choices .into_iter() .next()
    chat-->>and_then: parsed .choices .into_iter() .next() .and_then(|c| c.message.content)
    chat-->>context: parsed .choices .into_iter() .next() .and_then(|c| c.message.content)...
```

31 steps · 0 observed · 31 possible only

**Edges**

- calls → [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)
- calls → [`robby_render::util::text`](../../robby_render/util/text.md)

