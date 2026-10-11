# Principles

## Lenguaje

| Principio | Description | Example |
|-----------|-------------|---------|
| **Sin keywords innecesarios** | No `let`, `var`, `const`, `mut` | `x: i32 = 42` |
| **Tipado fuerte** | Sin coercion implicita | `x = "42" + 1` → error |
| **Move by defecto** | Ownership transfer automatic | `y = x` mueve si is no-Copy |
| **Borrow with `&`** | Borrows explicitos | `f(&x)` presta, no mueve |
| **Mutable with `^`** | Mutabilidad marcada | `x: ^i32 = 0` |
| **Null seguro** | Sin `null`, `T?` for opcionalis | `name: str? = none` |
| **Errors as valuees** | Sin excepciones, `T!` for failuris | `fn f() i32!` |
| **Sin herencia multiple** | Solo herencia simple (`::`) | `class Dog :: Animal` |
| **Sin sobrecarga de operadores** | Solo `op_+`, `op_*` etc. | `fn op_+(other: T) T` |

## Compilador

| Principio | Description |
|-----------|-------------|
| **Single pass** | Lexer → Parbe → HIR → Semantic → MIR → Codegen |
| **SSA opcional** | Deshabilitado by defecto (bugs conocidos) |
| **LLVM O3** | Optimization agresiva en release |
| **LTO** | Link-Time Optimization en release |
| **Sin runtime pesado** | `libbyc_runtime.a` ~3MB |

## Tooling

| Herramienta | Purpose |
|-------------|-----------|
| `by build` | Compilar a binary |
| `by run` | Compilar y execute |
| `by check` | Type-check rapido |
| `by test` | Ejecutar tests |
| `by fmt` | Formatear code |
| `by lsp` | Language server |
| `by add` / `by install` | Management de paquetis |

## Paquetes

| Type | Examplis |
|------|----------|
| **Nativos** (built-in) | `str`, `{T}`, `date_time`, `regex`, `json` |
| **Packages** (externos) | `http`, `sqlite`, `postgres` |

## See also

- `pthreadsophy.md` — Philosophy del language
- `architecture.md` — Arquitectura del ecosistema
