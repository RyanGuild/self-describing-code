/**
 * Custom HTML/SVG sequence diagram for Entrypoints / process views.
 * Replaces Mermaid so lifeline headers can be clickable portals and the
 * diagram reserves layout space instead of overlapping the hit-function grid.
 */

export interface SeqParticipant {
  id: string;
  label: string;
  /** Stable frame id when known (from FrameDetail.execution). */
  target?: string;
}

export interface SeqInteraction {
  index: number;
  from: string;
  to: string;
  label: string;
  observed: boolean;
  kind?: string;
}

export interface SeqModel {
  participants: SeqParticipant[];
  interactions: SeqInteraction[];
  stepCount: number;
  observedCount: number;
}

export interface SeqProcessLike {
  participants: { id: string; label: string; target?: string }[];
  events: {
    type: string;
    index?: number;
    from?: string;
    to?: string;
    label?: string;
    observed?: boolean;
    kind?: string;
  }[];
  step_count: number;
  observed_count: number;
}

const COL_W = 132;
const HEADER_H = 52;
const ROW_H = 30;
const PAD_X = 16;
const PAD_TOP = 8;
const MAX_LABEL = 42;

let markerSeq = 0;

export function modelFromProcess(process: SeqProcessLike): SeqModel {
  const interactions: SeqInteraction[] = [];
  for (const event of process.events) {
    if (event.type !== "interaction" || !event.from || !event.to) {
      continue;
    }
    const interaction: SeqInteraction = {
      index: event.index ?? interactions.length + 1,
      from: event.from,
      to: event.to,
      label: event.label ?? "",
      observed: Boolean(event.observed),
    };
    if (event.kind !== undefined) {
      interaction.kind = event.kind;
    }
    interactions.push(interaction);
  }
  return {
    participants: process.participants.map((p) => {
      const out: SeqParticipant = { id: p.id, label: p.label };
      if (p.target !== undefined) {
        out.target = p.target;
      }
      return out;
    }),
    interactions,
    stepCount: process.step_count,
    observedCount: process.observed_count,
  };
}

/** Best-effort parse of Mermaid sequenceDiagram (legacy frame payloads). */
export function modelFromMermaid(src: string): SeqModel | null {
  const participants: SeqParticipant[] = [];
  const interactions: SeqInteraction[] = [];
  let index = 0;
  for (const raw of src.split("\n")) {
    const line = raw.trim();
    if (!line || line === "sequenceDiagram" || line === "autonumber") {
      continue;
    }
    const part = /^participant\s+(\S+)(?:\s+as\s+(.+))?$/i.exec(line);
    if (part?.[1]) {
      const id = part[1];
      participants.push({
        id,
        label: (part[2] ?? id).trim(),
      });
      continue;
    }
    const msg = /^(\S+?)(?:-->>|->>|--x|->)(\S+)\s*:\s*(.*)$/.exec(line);
    if (msg?.[1] && msg[2]) {
      index += 1;
      interactions.push({
        index,
        from: msg[1],
        to: msg[2],
        label: msg[3] ?? "",
        observed: line.includes("->>") && !line.includes("-->>"),
      });
    }
  }
  if (participants.length === 0) {
    return null;
  }
  return {
    participants,
    interactions,
    stepCount: interactions.length,
    observedCount: interactions.filter((i) => i.observed).length,
  };
}

export function shortLabel(label: string, max = MAX_LABEL): string {
  if (label.length <= max) {
    return label;
  }
  const parts = label.split("::");
  if (parts.length > 1) {
    const tail = parts.slice(-2).join("::");
    if (tail.length <= max) {
      return tail;
    }
  }
  return `${label.slice(0, max - 1)}…`;
}

export type ResolveTarget = (participant: SeqParticipant) => string | undefined;

export function mountSequenceDiagram(
  host: HTMLElement,
  model: SeqModel,
  resolveTarget: ResolveTarget,
  onNavigate: (frameId: string) => void,
): void {
  host.classList.add("seq-diagram");
  host.replaceChildren();

  const cols = Math.max(1, model.participants.length);
  const rows = Math.max(1, model.interactions.length);
  const width = PAD_X * 2 + cols * COL_W;
  const height = PAD_TOP + HEADER_H + rows * ROW_H + 24;

  const indexOf = new Map(model.participants.map((p, i) => [p.id, i]));

  const inner = document.createElement("div");
  inner.className = "seq-inner";
  inner.style.width = `${width}px`;
  inner.style.minHeight = `${height}px`;

  const heads = document.createElement("div");
  heads.className = "seq-heads";
  heads.style.height = `${HEADER_H}px`;

  for (const participant of model.participants) {
    const target = participant.target ?? resolveTarget(participant);
    const head = document.createElement(target ? "button" : "div");
    head.className = `seq-head${target ? " is-portal" : ""}`;
    head.style.width = `${COL_W}px`;
    head.title = participant.label;
    head.textContent = shortLabel(participant.label);
    if (target && head instanceof HTMLButtonElement) {
      head.type = "button";
      head.addEventListener("click", (e) => {
        e.stopPropagation();
        onNavigate(target);
      });
    }
    heads.appendChild(head);
  }
  inner.appendChild(heads);

  const svgNs = "http://www.w3.org/2000/svg";
  const markerId = `seq-arrowhead-${++markerSeq}`;
  const svg = document.createElementNS(svgNs, "svg");
  svg.setAttribute("class", "seq-svg");
  svg.setAttribute("width", String(width));
  svg.setAttribute("height", String(height - HEADER_H));
  svg.setAttribute("viewBox", `0 0 ${width} ${height - HEADER_H}`);
  svg.setAttribute("role", "img");
  svg.setAttribute(
    "aria-label",
    `Sequence with ${model.participants.length} lifelines and ${model.interactions.length} calls`,
  );

  const defs = document.createElementNS(svgNs, "defs");
  const marker = document.createElementNS(svgNs, "marker");
  marker.setAttribute("id", markerId);
  marker.setAttribute("markerWidth", "8");
  marker.setAttribute("markerHeight", "8");
  marker.setAttribute("refX", "7");
  marker.setAttribute("refY", "3");
  marker.setAttribute("orient", "auto");
  const tip = document.createElementNS(svgNs, "path");
  tip.setAttribute("d", "M0,0 L7,3 L0,6 Z");
  tip.setAttribute("class", "seq-arrowhead");
  marker.appendChild(tip);
  defs.appendChild(marker);
  svg.appendChild(defs);

  const bodyH = height - HEADER_H;
  for (let i = 0; i < cols; i++) {
    const x = PAD_X + i * COL_W + COL_W / 2;
    const line = document.createElementNS(svgNs, "line");
    line.setAttribute("x1", String(x));
    line.setAttribute("y1", "0");
    line.setAttribute("x2", String(x));
    line.setAttribute("y2", String(bodyH));
    line.setAttribute("class", "seq-lifeline");
    svg.appendChild(line);
  }

  model.interactions.forEach((msg, row) => {
    const fromIdx = indexOf.get(msg.from);
    const toIdx = indexOf.get(msg.to);
    if (fromIdx === undefined || toIdx === undefined) {
      return;
    }
    const y = 18 + row * ROW_H;
    const x1 = PAD_X + fromIdx * COL_W + COL_W / 2;
    const x2 = PAD_X + toIdx * COL_W + COL_W / 2;
    const self = fromIdx === toIdx;

    if (self) {
      const path = document.createElementNS(svgNs, "path");
      const r = 14;
      path.setAttribute(
        "d",
        `M ${x1} ${y} C ${x1 + r * 2} ${y - 6}, ${x1 + r * 2} ${y + 14}, ${x1} ${y + 12}`,
      );
      path.setAttribute("class", msg.observed ? "seq-arrow is-observed" : "seq-arrow");
      path.setAttribute("fill", "none");
      path.setAttribute("marker-end", `url(#${markerId})`);
      svg.appendChild(path);
    } else {
      const line = document.createElementNS(svgNs, "line");
      line.setAttribute("x1", String(x1));
      line.setAttribute("y1", String(y));
      line.setAttribute("x2", String(x2));
      line.setAttribute("y2", String(y));
      line.setAttribute("class", msg.observed ? "seq-arrow is-observed" : "seq-arrow");
      line.setAttribute("marker-end", `url(#${markerId})`);
      svg.appendChild(line);
    }

    const label = document.createElementNS(svgNs, "text");
    const midX = self ? x1 + 28 : (x1 + x2) / 2;
    label.setAttribute("x", String(midX));
    label.setAttribute("y", String(self ? y - 2 : y - 6));
    label.setAttribute("text-anchor", "middle");
    label.setAttribute("class", "seq-msg");
    label.textContent = `${msg.index}. ${shortLabel(msg.label, 36)}`;
    svg.appendChild(label);
  });

  inner.appendChild(svg);
  host.appendChild(inner);
}
