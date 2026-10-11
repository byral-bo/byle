#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== Byle pre-release gate =="

if ! command -v cargo-audit >/dev/null 2>&1; then
    echo "ERROR: cargo-audit 0.22.2 is required" >&2
    echo "Run: cargo install cargo-audit --version 0.22.2 --locked" >&2
    exit 2
fi

echo "-- format --"
cargo fmt --all -- --check
git diff --check

echo "-- workspace metadata --"
cargo metadata --no-deps --locked --format-version 1 >/dev/null

echo "-- migration hygiene --"
if find . -path './.git' -prune -o -path './target' -prune -o -path './dist' -prune \
    -type f \( -name '*.ky' -o -name '*.kyx' -o -name 'ky.toml' -o -name 'kyc_*' \) -print \
    | grep -q .; then
    echo "ERROR: legacy Kyle files found" >&2
    exit 1
fi
if rg -n -i --glob '!target/**' --glob '!.git/**' --glob '!dist/**' \
    --glob '!scripts/pre-release-check.sh' \
    'kyle programming language|kyc_|kyui|kycPath|\.kyx\b|\.ky\b|ky\.toml|test-ky|\bKL\b|\bkl_' . >/dev/null; then
    echo "ERROR: legacy Kyle references found" >&2
    exit 1
fi

echo "-- locked check --"
cargo check --workspace --locked

echo "-- workspace tests --"
cargo test --workspace --locked

echo "-- strict clippy --"
cargo clippy --workspace --locked -- -D warnings

echo "-- security audit --"
cargo audit --deny warnings

echo "-- release build --"
cargo build --release --bin by --locked
cargo build --release -p byc_runtime --locked

echo "-- end-to-end --"
./scripts/test-by.sh ./target/release/by

echo "== pre-release gate: PASS =="
