#!/usr/bin/env bash
# Regenerate every committed artifact from the IR.
#
# This is the whole pipeline today. It will grow a real extractor in M1; for
# now the IR is hand-authored, which is exactly what we want while the schema
# is still moving.
set -euo pipefail
cd "$(dirname "$0")/.."

IR="${1:-fixtures/m0/hello.ir.json}"
OUT_MD="${2:-docs/robby-self-model.md}"
OUT_MMD=".robby/m0"

echo "== check =="
cargo run -q -p robby-cli -- check --ir "$IR"

echo "== render =="
cargo run -q -p robby-cli -- render --ir "$IR" --out "$OUT_MMD"

echo "== report =="
cargo run -q -p robby-cli -- report --ir "$IR" --out "$OUT_MD"

echo "done: $OUT_MD"
