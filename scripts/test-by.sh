#!/usr/bin/env bash
# E2E gate — compila y ejecuta los tests .by con el binario release.
# Uso: ./scripts/test-by.sh [BY_BIN]
# Sale con código != 0 si algún test falla. Debe correr desde la raíz del repo.
set -uo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT" || exit 1

BY="${1:-${BY:-$ROOT/target/release/by}}"

if [[ "$BY" != /* ]]; then
    BY="$ROOT/$BY"
fi

if [ ! -x "$BY" ]; then
    echo "ERROR: binario 'by' no encontrado en $BY (corre: cargo build --release --bin by --locked)"
    exit 2
fi

RUNTIME_LIB="$ROOT/target/release/libbyc_runtime.a"
if [ ! -f "$RUNTIME_LIB" ]; then
    echo "ERROR: runtime no encontrado en $RUNTIME_LIB (corre: cargo build --release -p byc_runtime --locked)"
    exit 2
fi

PASS=0
FAIL=0
FAILED_TESTS=()

run_one() {
    local name="$1"
    shift
    local output
    if output=$("$@" 2>&1); then
        PASS=$((PASS + 1))
        echo "  PASS  $name"
    else
        FAIL=$((FAIL + 1))
        FAILED_TESTS+=("$name")
        echo "  FAIL  $name"
        echo "$output" | sed 's/^/        | /' | tail -15
    fi
}

run_expected_fail() {
    local name="$1"
    shift
    local output
    if output=$("$@" 2>&1); then
        FAIL=$((FAIL + 1))
        FAILED_TESTS+=("$name (debía fallar)")
        echo "  FAIL  $name (aceptado incorrectamente)"
    else
        PASS=$((PASS + 1))
        echo "  PASS  $name (rechazado correctamente)"
    fi
}

echo "== by e2e suite =="
echo "binario: $BY"
echo

echo "-- tests/syntax --"
for f in tests/syntax/*.by; do
    [ -e "$f" ] || continue
    run_one "$f" "$BY" run "$f"
done

echo "-- tests/std --"
for f in tests/std_*.by; do
    [ -e "$f" ] || continue
    run_one "$f" "$BY" run "$f"
done

echo "-- tests/regression --"
if compgen -G "tests/regression/*.by" > /dev/null; then
    for f in tests/regression/*.by; do
        run_one "$f" "$BY" run "$f"
    done
else
    echo "  (vacío)"
fi

echo "-- tests/negative --"
for f in tests/negative/*.by; do
    [ -e "$f" ] || continue
    run_expected_fail "$f" "$BY" check "$f"
done

echo "-- tests root --"
for f in tests/syntax_comprehensive*.by; do
    [ -e "$f" ] || continue
    run_one "$f" "$BY" run "$f"
done

echo "-- std checks --"
for f in std/*.by; do
    [ -e "$f" ] || continue
    run_one "$f" "$BY" check "$f"
done

echo "-- examples --"
for f in examples/*.by; do
    [ -e "$f" ] || continue
    run_one "$f" "$BY" run "$f"
done

echo "-- project lifecycle --"
PROJECT_TMP="$(mktemp -d "${TMPDIR:-/tmp}/byle-project.XXXXXX")"
cleanup_e2e() {
    rm -rf "$PROJECT_TMP" "${PACKAGE_TMP:-}"
}
trap cleanup_e2e EXIT
PROJECT_DIR="$PROJECT_TMP/app"
if "$BY" new bare "$PROJECT_DIR" >/dev/null 2>&1 \
    && mkdir -p "$PROJECT_DIR/tests" \
    && cp tests/fixtures/project_test.by "$PROJECT_DIR/tests/" \
    && (cd "$PROJECT_DIR" && "$BY" check app.by >/dev/null) \
    && (cd "$PROJECT_DIR" && "$BY" build app.by >/dev/null) \
    && (cd "$PROJECT_DIR" && "$BY" test >/dev/null) \
    && (cd "$PROJECT_DIR" && ./target/debug/app >/dev/null); then
    PASS=$((PASS + 1))
    echo "  PASS  generated project: new/check/build/test/run"
else
    FAIL=$((FAIL + 1))
    FAILED_TESTS+=("generated project lifecycle")
    echo "  FAIL  generated project: new/check/build/test/run"
fi

echo "-- local package registry --"
PACKAGE_TMP="$(mktemp -d "${TMPDIR:-/tmp}/byle-registry.XXXXXX")"
cp -R tests/fixtures/registry/demo "$PACKAGE_TMP/"
tar -czf "$PACKAGE_TMP/demo/1.0.0.tar.gz" \
    -C "$PACKAGE_TMP/demo/1.0.0" by.toml src
PACKAGE_DIR="$PACKAGE_TMP/app"
if "$BY" new bare "$PACKAGE_DIR" >/dev/null 2>&1 \
    && (cd "$PACKAGE_DIR" && BYLE_REGISTRY="file://$PACKAGE_TMP" BY_HOME="$PACKAGE_TMP/home" \
        "$BY" add demo@1.0.0 >/dev/null) \
    && test -f "$PACKAGE_DIR/by.lock" \
    && test -f "$PACKAGE_DIR/std/demo/lib.by" \
    && (cd "$PACKAGE_DIR" && BYLE_REGISTRY="file://$PACKAGE_TMP" BY_HOME="$PACKAGE_TMP/home" \
        "$BY" install >/dev/null); then
    PASS=$((PASS + 1))
    echo "  PASS  local registry: add/resolve/lock/install"
else
    FAIL=$((FAIL + 1))
    FAILED_TESTS+=("local package registry")
    echo "  FAIL  local registry: add/resolve/lock/install"
fi

echo
echo "=================================="
echo "PASS: $PASS   FAIL: $FAIL"
if [ "$FAIL" -gt 0 ]; then
    echo "Fallidos:"
    for t in "${FAILED_TESTS[@]}"; do
        echo "  - $t"
    done
    exit 1
fi
if [ "$PASS" -eq 0 ]; then
    echo "ERROR: no se ejecutó ninguna prueba E2E"
    exit 1
fi
echo "OK"
