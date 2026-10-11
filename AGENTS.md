# Byle Monorepo — AI Agent Context

> Single entry-point for AI agents and team members working on the Byle codebase.
> Current focus: **Backend package ecosystem** (see [`BACKEND_PACKAGES_PLAN.md`](docs/11-project/backend-packages-plan.md))
> See also: [`BUILD.md`](BUILD.md) · [`docs/11-project/benchmarks.md`](docs/11-project/benchmarks.md) · [`docs/packages/`](docs/packages/) · [`tests/SYNTAX_CHECKLIST.md`](tests/SYNTAX_CHECKLIST.md)

## Quick Reference

```bash
# Build
cargo build --release --bin by

# Run
by run examples/hello.by

# Test
cargo test --workspace
for f in tests/syntax/*.by; do by run "$f"; done
for f in tests/std_*.by; do by run "$f"; done

# Check package
by check packages/http/src/lib.by

# Run package tests
by test packages/http/tests/test_http.by

# Install package
by add http

# Create project
by new myapp
```

## Architecture Overview

```
by/
├── crates/                  ← Rust compiler & tooling
│   ├── byc_frontend/        → Lexer + parser
│   ├── byc_hir/             → HIR desugaring
│   ├── byc_semantic/        → Type checker, borrow analysis
│   ├── byc_mir/             → MIR lowering, SSA, optimizations
│   ├── byc_backend/         → LLVM codegen, runtime linkage
│   ├── byc_driver/          → Compilation pipeline
│   ├── byc_cli/             → CLI binary (`by`)
│   ├── byc_runtime/         → Runtime library (Rust) — 190+ extern "C" functions
│   └── byc_tools/           → LSP, formatter, package manager
│
├── packages/                → Byle installable packages
├── (external) byle-vscode/  → VS Code extension
├── docs/                    → Language docs, specs, plans
│   ├── 04-standard-library/ → 21 std modules (native)
│   ├── packages/            → 9 package docs
│   └── 11-project/          → Roadmaps, plans
├── tests/                   → Syntax tests, package tests
├── examples/                → .by + .byx example programs
└── scripts/                 → install.sh, dev-install.sh
```

## Backend Ecosystem

### Standard Library (native — `use std.<module>`)

| Module | File | Status |
|--------|------|--------|
| `result` | `docs/04-standard-library/result.md` | ✅ Runtime (language) |
| `json` | `docs/04-standard-library/json.md` | ✅ Runtime (`by_json_*`) |
| `time` | `docs/04-standard-library/time.md` | ✅ Runtime (`by_datetime_*`) |
| `fs` | `docs/04-standard-library/fs.md` | ✅ Runtime (`by_fs_*`) |
| `path` | `docs/04-standard-library/path.md` | ✅ Runtime (`by_path_*`) |
| `str` | `docs/04-standard-library/str.md` | ✅ Runtime (`by_str_*`) |
| `math` | `docs/04-standard-library/math.md` | ✅ Runtime (`by_pow`, etc.) |
| `io` | `docs/04-standard-library/io.md` | ✅ Runtime (`by_print*`) |
| `net` | `docs/04-standard-library/net.md` | ✅ Runtime (`by_tcp_*`) |
| `random` | `docs/04-standard-library/random.md` | ✅ Runtime (`by_random_bytes`) |
| `regex` | `docs/04-standard-library/regex.md` | ✅ Runtime (`by_regex_*`) |
| `thread` | `docs/04-standard-library/thread.md` | ✅ Runtime (`by_spawn_thread`) |
| `sync` | `docs/04-standard-library/sync.md` | ✅ Runtime (`by_mutex_*`) |
| `crypto` | `docs/04-standard-library/crypto.md` | ✅ Runtime (sha256, base64, uuid) |
| `process` | `docs/04-standard-library/process.md` | ✅ Runtime (`by_getenv`, libc) |
| `testing` | `docs/04-standard-library/testing.md` | ✅ Runtime (`by_assert*`) |
| `log` | `docs/04-standard-library/log.md` | ✅ Runtime (`by_log_*` + wrapper) |
| `cli` | `docs/04-standard-library/cli.md` | ✅ Runtime (`by_cli_*`) |
| `csv` | `docs/04-standard-library/csv.md` | ✅ Runtime (`by_csv_*`, handle-based) |
| `url` | `docs/04-standard-library/url.md` | 🟡 Extend existing runtime |
| `bytes` | `docs/04-standard-library/bytes.md` | 🟡 Extend existing runtime |

### Packages (installable — `by add`)

| Package | Doc | Status |
|---------|-----|--------|
| `http` | `docs/packages/http.md` | 🟡 Rewrite client (TCP direct), sub-modules |
| `postgres` | `docs/packages/postgres.md` | 🟡 Add typed params, transactions |
| `sqlite` | `docs/packages/sqlite.md` | 🟡 Add Database/Statement classes |
| `env` | `docs/packages/env.md` | 🟡 Add .env loader, typed accessors |
| `crypto` | `docs/packages/crypto.md` | ❌ New (password hash, JWT, HMAC) |
| `config` | `docs/packages/config.md` | ❌ New (YAML/TOML loader) |
| `compress` | `docs/packages/compress.md` | ❌ New (gzip) |
| `mail` | `docs/packages/mail.md` | ❌ New (SMTP) |

---

## Documentation Map

| Document | Location | Content |
|----------|----------|---------|
| **AGENTS.md** (this) | `AGENTS.md` | Main entry point |
| **Build guide** | `BUILD.md` | Build, install, test |
| **Packages** | `docs/packages/` | Package specifications and registry metadata |
| **Backend plan** | `docs/11-project/backend-packages-plan.md` | Complete implementation plan with tasks |
| **Std library docs** | `docs/04-standard-library/` | 21 native module docs |
| **Package docs** | `docs/packages/` | 9 installable package docs |
| **Syntax reference** | `docs/15-byle-syntax-reference.md` | .by language reference |
| **Roadmap** | `docs/11-project/roadmap.md` | Overall project state |
| **Remaining work** | `docs/11-project/remaining-work.md` | Bugs & features |
| **Runtime functions** | `crates/byc_runtime/src/` | 190+ `extern "C"` functions |
| **BYUI docs** | `docs/03-language/ui/` | UI framework (paused) |

---

## Current State

### ✅ Core Language — Stable

| Component | Status |
|-----------|--------|
| Lexer + Parser | ✅ |
| Type system (generics, enums, classes) | ✅ |
| Borrow checker | ✅ |
| MIR (lowering, SSA, optimizations) | ✅ |
| LLVM codegen | ✅ |
| Collections: `[T]`, `{K:V}`, `set{T}`, `queue{T}`, `stack{T}`, `deque{T}` | ✅ |
| Orthogonal types: `^` `&` `?` `!` | ✅ |
| Module system: `use X.Y` | ✅ |
| Package manager: `by add`, `by.toml` | ✅ |
| Syntax tests: 13/13 passing | ✅ |
| Workspace tests: 117+ | ✅ |

### 🟢 Runtime — Rich FFI Surface

190+ `extern "C"` functions available in `byc_runtime/src/`:
- `by_tcp_*` — TCP networking
- `by_fs_*` — File system
- `by_str_*` — String manipulation
- `by_list_*`, `by_dict_*`, `by_set_*`, `by_queue_*`, `by_stack_*`, `by_deque_*`
- `by_json_*` — JSON parse/stringify
- `by_datetime_*`, `by_date_*`, `by_time_*`, `by_duration_*`
- `by_regex_*` — Regular expressions
- `by_sha256`, `by_base64_encode`, `by_uuid_v4` — Crypto
- `by_mutex_*`, `by_atomic_*`, `by_channel_*` — Sync
- `by_spawn_thread`, `by_join_thread` — Threading
- `by_ws_*` — WebSocket frames

### ⏸️ BYUI — Paused

UI framework development is paused. Existing code at `packages/ui/`, `crates/byc_ui/`.
All 30+ components, web backend, JS runtime are functional but not in active development.

---

## Implementation Plan

See [`docs/11-project/backend-packages-plan.md`](docs/11-project/backend-packages-plan.md) for complete task breakdown.

### Phase 1: Std Core Wrappers (16 modules)
Create `use std.X` Byle wrappers for existing runtime functions.

### Phase 2: Std New Runtime (5 modules)
Implement `by_log_*`, `by_cli_*`, `by_csv_*`, extend `url.rs` and `bytes.rs`.

### Phase 3: Package Improvements (4 packages)
Rewrite `http` (TCP client), improve `postgres`, `sqlite`, `env`.

### Phase 4: New Packages (4 packages)
Implement `crypto`, `config` (YAML/TOML), `compress` (gzip), `mail` (SMTP).

---

## Development Workflow

### Backend Package Development

1. **Check runtime** — Verify `byc_runtime/src/` has needed extern fns, or add them
2. **Register codegen** — Add new extern fns to `byc_backend/src/codegen/function.rs`
3. **Create Byle wrapper** — Write `packages/<name>/src/lib.by` or `packages/std/<name>.by`
4. **Build** — `./scripts/dev-install.sh`
5. **Test** — `by test packages/<name>/tests/`

### Adding a New Runtime Function

1. Add Rust function in `byc_runtime/src/<module>.rs` with `#[no_mangle] pub extern "C" fn by_*`
2. Export it in `byc_runtime/src/lib.rs`
3. In Byle: `@link "c" extern fn by_*(...) <type>`
4. Rebuild: `cargo build --release --bin by`

---

## ✅ All Known Compiler Bugs Fixed

| Bug | Status |
|-----|--------|
| `return ok(val)` / `return error(msg)` fallible return | ✅ FIXED |
| `match result: ok(n): n error(e): 0` (T! from function) | ✅ FIXED |
| `^[Token].push()` inside loops → `.len()==0` | ✅ WORKS |
| `^[str].pop()` / `.first()` / `.last()` / `.get()` returned garbage | ✅ FIXED |
| `^[Token].pop()` returned garbage | ✅ FIXED |
| String `>=` `<=` `>` `<` comparison wrong | ✅ FIXED |
| `.find()` → link error | ✅ FIXED |
| `.split()` returned wrong substrings | ✅ FIXED |
| `by_clone_str` → link error crash | ✅ FIXED |
| `_name`/`__name` scope resolver | ✅ FIXED |
| `&[Token]`/`^[Token]` function params → LLVM type mismatch | ✅ FIXED |
| `&[T]` slice creation `arr[0..2]` → returns garbage | ✅ FIXED |
| Field access on classes returns `i32` | ✅ FIXED |
| `str_builder_free` crash | ✅ Fixed |
| `_call` SSA verify error in large files | ✅ FIXED |
| `!` error propagation operator | ✅ FIXED |
| `&str` prelude functions (fs, uuid, etc.) SIGSEGV | ✅ FIXED |
