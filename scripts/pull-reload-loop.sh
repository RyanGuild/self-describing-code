#!/usr/bin/env bash
# Continuously wait → pull origin/main → rebuild site when HEAD moves.
# Intended for a local Mac checkout that hosts the Prezi viewer.
#
# Usage:
#   ./scripts/pull-reload-loop.sh
#   INTERVAL_SEC=120 PORT=5174 ./scripts/pull-reload-loop.sh
#   ./scripts/pull-reload-loop.sh --once
#
# Env:
#   INTERVAL_SEC   seconds between pulls (default: 120)
#   PORT           Vite port to keep serving (default: 5174)
#   HOST           Vite bind host (default: 127.0.0.1)
#   ROBBY_ROOT     repo root (default: parent of scripts/)
#   SKIP_RELOAD=1  pull only; do not rebuild or touch Vite
#   AUTO_STASH=1   stash dirty worktree before checkout/pull (pop after)

set -euo pipefail

INTERVAL_SEC="${INTERVAL_SEC:-120}"
PORT="${PORT:-5174}"
HOST="${HOST:-127.0.0.1}"
SKIP_RELOAD="${SKIP_RELOAD:-0}"
AUTO_STASH="${AUTO_STASH:-0}"
ONCE=0

for arg in "$@"; do
  case "$arg" in
    --once) ONCE=1 ;;
    -h|--help)
      sed -n '2,20p' "$0"
      exit 0
      ;;
    *)
      echo "unknown arg: $arg" >&2
      exit 2
      ;;
  esac
done

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROBBY_ROOT:-$(cd "$SCRIPT_DIR/.." && pwd)}"
cd "$ROOT"

log() { printf '[%s] %s\n' "$(date '+%Y-%m-%d %H:%M:%S %Z')" "$*"; }

dirty() {
  [ -n "$(git status --porcelain 2>/dev/null)" ]
}

ensure_vite() {
  if curl -sf "http://${HOST}:${PORT}/" >/dev/null 2>&1; then
    return 0
  fi
  log "Vite not responding on http://${HOST}:${PORT}/ — starting apps/site"
  (
    cd "$ROOT/apps/site"
    if [ ! -d node_modules ]; then
      npm install
    fi
    # Prefer npm run; fall back if package scripts differ
    nohup npm run dev -- --host "$HOST" --port "$PORT" \
      >"$ROOT/.robby/vite-${PORT}.log" 2>&1 &
    echo $! >"$ROOT/.robby/vite-${PORT}.pid"
  )
  for _ in $(seq 1 30); do
    if curl -sf "http://${HOST}:${PORT}/" >/dev/null 2>&1; then
      log "Vite up at http://${HOST}:${PORT}/"
      return 0
    fi
    sleep 0.5
  done
  log "WARN: Vite failed to become ready; see .robby/vite-${PORT}.log"
  return 1
}

reload_site() {
  if [ "$SKIP_RELOAD" = "1" ]; then
    log "SKIP_RELOAD=1 — not rebuilding"
    return 0
  fi
  mkdir -p "$ROOT/.robby"
  log "Building robby-cli…"
  cargo build -p robby-cli
  local bin="$ROOT/target/debug/robby"
  if [ ! -x "$bin" ]; then
    log "ERROR: missing $bin"
    return 1
  fi
  if [ ! -f "$ROOT/.robby/robby.ir.json" ]; then
    log "No IR — extracting…"
    "$bin" extract --out "$ROOT/.robby/robby.ir.json"
  fi
  log "Running robby web → apps/site/public"
  "$bin" web --ir "$ROOT/.robby/robby.ir.json" --out "$ROOT/apps/site/public"
  ensure_vite || true
  local title
  title="$(curl -sf "http://${HOST}:${PORT}/" | sed -n 's/.*<title>\([^<]*\)<\/title>.*/\1/p' | head -1 || true)"
  log "Viewer: http://${HOST}:${PORT}/  title=${title:-unknown}"
}

pull_once() {
  local before after stash_ref=""
  log "fetch origin/main"
  git fetch origin main

  if [ "$(git rev-parse --abbrev-ref HEAD)" != "main" ]; then
    if dirty && [ "$AUTO_STASH" != "1" ]; then
      log "WARN: on $(git rev-parse --abbrev-ref HEAD), dirty tree — fetch only (set AUTO_STASH=1 or switch to clean main)"
      log "origin/main=$(git rev-parse --short origin/main) $(git log -1 --oneline origin/main)"
      return 0
    fi
    if dirty && [ "$AUTO_STASH" = "1" ]; then
      log "AUTO_STASH: stashing dirty WIP"
      git stash push -u -m "pull-reload-loop auto-stash $(date -u +%Y%m%dT%H%M%SZ)"
      stash_ref="stash@{0}"
    fi
    log "checkout main"
    git checkout main
  elif dirty; then
    if [ "$AUTO_STASH" = "1" ]; then
      log "AUTO_STASH: stashing dirty WIP on main"
      git stash push -u -m "pull-reload-loop auto-stash $(date -u +%Y%m%dT%H%M%SZ)"
      stash_ref="stash@{0}"
    else
      log "WARN: dirty main — cannot pull --ff-only (set AUTO_STASH=1)"
      return 0
    fi
  fi

  before="$(git rev-parse HEAD)"
  if ! git pull --ff-only origin main; then
    log "ERROR: pull --ff-only failed"
    return 1
  fi
  after="$(git rev-parse HEAD)"
  log "HEAD=$after $(git log -1 --oneline)"

  if [ "$before" != "$after" ]; then
    log "main advanced $before → $after — reloading"
    reload_site
  else
    log "main unchanged"
    # Still ensure viewer is up
    if [ "$SKIP_RELOAD" != "1" ]; then
      ensure_vite || true
    fi
  fi

  if [ -n "$stash_ref" ]; then
    log "NOTE: left stash in place ($stash_ref); restore manually when ready"
  fi
}

log "repo=$ROOT interval=${INTERVAL_SEC}s port=$PORT once=$ONCE"
if [ "$ONCE" = "1" ]; then
  pull_once
  exit 0
fi

# First tick immediately, then sleep between pulls
while true; do
  pull_once || log "cycle failed; will retry after sleep"
  log "sleep ${INTERVAL_SEC}s…"
  sleep "$INTERVAL_SEC"
done
