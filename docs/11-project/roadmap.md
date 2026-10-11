# Byle — Roadmap de Desarrollo

> Current focus: **Backend package ecosystem** (see [`backend-packages-plan.md`](backend-packages-plan.md))
> UI Framework: ⏸️ **Paused** (see `docs/03-language/ui/`)

---

## Estado Actual

| Área | Estado |
|------|--------|
| **Compilador** (Fases 1-17) | ✅ Completo |
| **Runtime** (memoria, strings, colecciones, TCP, JSON, crypto, regex, threads, sync) | ✅ Completo (190+ extern fns) |
| **Borrow checker** | ✅ Completo |
| **Cross-platform** (macOS, Linux, Windows) | ✅ Completo |
| **Tooling** (LSP, formatter, VS Code, package manager) | ✅ Completo |
| **Core language** (generics, enums, classes, modules) | ✅ Completo |
| **Error handling** (`T!`, `T?`, `!` operator) | ✅ Completo |
| **FFI** (`extern fn`, `@link`, `ptr`) | ✅ Completo |

---

## Fases de Desarrollo Backend

### 🔴 FASE 1: Std Core Wrappers (19 módulos) — ✅ 21 wrappers creados en `packages/std/`

Crear wrappers Byle para las 190+ funciones runtime existentes. Todos compilan (`by check`); smoke-tested en conjunto.

| Módulo | Estado | Notas |
|--------|:------:|-------|
| `std::result` | ✅ Wrapper | `ok`/`error`/`some`/`none` son builtins |
| `std::json` | ✅ Wrapper | `by_json_*`, `by_struct_to_json` |
| `std::time` | ✅ Wrapper | `by_datetime_*`, `by_date_*`, `by_duration_*` |
| `std::fs` | ✅ Wrapper | `by_fs_*` |
| `std::path` | ✅ Wrapper | `by_path_*` |
| `std::str` | ✅ Wrapper | `by_str_*`, `Str` class |
| `std::math` | ✅ Wrapper | `by_pow`, etc. |
| `std::io` | ✅ Wrapper | `by_print*`, `by_input*` |
| `std::net` | ✅ Wrapper | `by_tcp_*` |
| `std::random` | ✅ Wrapper | `Random.bytes`, `Random.int` |
| `std::regex` | ✅ Wrapper | `by_regex_*` |
| `std::thread` | ✅ Wrapper | `by_spawn_thread` |
| `std::sync` | ✅ Wrapper | `Mutex`/`AtomicI64`/`AtomicBool`/`Channel` (+ `tests/std_sync.by`) |
| `std::crypto` | ✅ Wrapper | sha256, base64, uuid |
| `std::process` | ✅ Wrapper | `by_getenv`, libc |
| `std::testing` | ✅ Wrapper | `by_assert*`, `Assert` |
| `std::bytes` | ✅ Implementado | `by_bytes_*`, `by_buffer_*` `tests/std_bytes.by` |
| `std::cli` | ✅ Implementado | `by_cli_*`, `std/cli.by` `tests/std_cli.by` |
| `std::csv` | ✅ Implementado | `by_csv_*`, `std/csv.by` `tests/std_csv.by` |
| `std::url` | ✅ Wrapper | `by_url_*` |
| `std::log` | ✅ Wrapper | `by_log_*`, `Log` class |

**Total:** 21/21 std modules documented. Implementados con wrapper+test: result, json, time, fs, path, str, math, io, net, random, regex, thread, sync, crypto, process, testing, bytes, cli, csv.

### 🟡 FASE 2: Std New Runtime (5 módulos)

Implementar runtime functions + wrappers Byle.

| Módulo | Runtime | Byle Wrapper | Tests | Prioridad |
|--------|:-------:|:------------:|:-----:|:---------:|
| `std::log` | 🟡 `log.rs` | ❌ | ❌ | 🔴 Alta |
| `std::cli` | ✅ `by_cli_*` | ✅ `std/cli.by` | ✅ `tests/std_cli.by` | 🔴 Alta |
| `std::csv` | ✅ `by_csv_*` | ✅ `std/csv.by` | ✅ `tests/std_csv.by` | 🟡 Media |
| `std::url` | 🟡 Extender | ❌ | ❌ | 🟡 Media |
| `std::bytes` | ✅ `by_bytes_*` | ✅ `std/bytes.by` | ✅ `tests/std_bytes.by` | 🟡 Media |

### 🟡 FASE 3: Package Improvements (4 packages)

Mejorar packages existentes con funcionalidades faltantes.

| Package | Estado | Pendiente |
|---------|:------:|-----------|
| `http` | 🟡 | Cliente TCP directo (sin curl), sub-módulos, middleware |
| `postgres` | 🟡 | Queries parametrizados, binds tipados, transacciones |
| `sqlite` | 🟡 | Database/Statement/ResultSet classes |
| `env` | 🟡 | .env loader, typed accessors, setenv/unset |

### 🟢 FASE 4: New Packages (4 packages)

Crear packages desde cero con runtime Rust + Byle wrapper.

| Package | Runtime | Byle Wrapper | Tests | Prioridad |
|---------|:-------:|:------------:|:-----:|:---------:|
| `crypto` | ❌ | ❌ | ❌ | 🟡 Media |
| `config` (YAML/TOML) | ❌ | ❌ | ❌ | 🟡 Media |
| `compress` (gzip) | ❌ | ❌ | ❌ | 🟢 Baja |
| `mail` (SMTP) | ❌ | ❌ | ❌ | 🟢 Baja |

---

## ⏸️ UI Framework — Paused

UI development (`.byx`, web/desktop/iOS/TUI backends) is paused.
Existing code at `packages/ui/` and `crates/byc_ui/` remains functional but not in active development.

See `docs/03-language/ui/README.md` for full UI documentation.

---

## Testing

```bash
# Rust tests
cargo test --workspace --exclude byc_runtime_wasm

# Build release binary
cargo build --release --bin by

# Syntax tests
for f in tests/syntax/*.by; do by run "$f"; done

# Package tests
by test packages/http/tests/test_http.by

# Check package
by check packages/http/src/lib.by
```

---

*Última actualización: 2026-07-30 · Focus: Backend packages*
