# `robby__site/src/viewer/portalButton`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.function` |
| location | [`apps/site/src/viewer.ts:634`](../../../../../apps/site/src/viewer.ts#L634) |
| provenance | extractor · confidence 1.00 |

```rust
function portalButton(portal: Portal): HTMLButtonElement {
  const btn = document.createElement("button");
  btn.type = "button";
  btn.className = "portal" + (portal.target ? "" : " unresolved");
  btn.innerHTML =
    `<span class="pk">${escapeHtml(portal.kind_label)}</span>` +
    escapeHtml(portal.label);
  btn.title = portal.target_name || portal.label;
  if (portal.target) {
    const target = portal.target;
    btn.addEventListener("click", (e) => {
      e.stopPropagation();
      flyTo(target);
    });
  }
  return btn;
}
```

```mermaid
sequenceDiagram
    autonumber
    participant portalButton as portalButton()
    participant createElement as createElement
    participant escapeHtml as escapeHtml
    participant addEventListener as addEventListener
    self-->>createElement: 
    self-->>escapeHtml: 
    alt if
    self-->>addEventListener: 
    self-->>fn: 
    end
    return-->>caller: 
```

5 steps · 0 observed · 5 possible only

**Edges**

- calls → [`robby__site/src/viewer/escapeHtml`](escapehtml.md)

## References

- calls → [`robby__site/src/viewer`](index.md) — escapeHtml

