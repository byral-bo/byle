# Byle Self-Hosting Status

## 3 Rust Compiler Fixes (this session)

| Fix | Archivo | Líneas cambiadas |
|-----|---------|:----------------:|
| `str` es Copy — no requiere `&` al pasar a funciones | `byc_semantic/type_checker.rs:836` | 1 |
| Struct → `i64` para `extern fn` (SSA + non-SSA) | `byc_backend/codegen.rs:820,3635` | 26 |
| `s.upper()`, `s.lower()`, `str.get(i)` aliases | `byc_mir/lower.rs:3997,4006,4333,5446,7494` | 28 |

**Todos los 181 tests Rust pasan.**

---

## Byle Components (`runtimes/by/`)

| Componente | `by check` | `by run` | Estado real |
|-----------|:----------:|:--------:|-------------|
| `token.by` | ✅ | ✅ | Tipos de token (enum + class) |
| `alloc.by` | ✅ | ✅ | `extern fn` wrappers (malloc/free) |
| `codegen.by` | ✅ | ✅ | Genera LLVM IR para add/max/fact |
| `compiler.by` | ✅ | ✅ | Pipeline demo hardcodeado (add/fact) |
| `bootstrap.by` | ✅ | ✅ | Demo 2+3=5 (NO es self-hosting real) |
| `ky2c.by` | ✅ | ⚠️ | **Parcial**: genera C pero bugs de runtime lo bloquean |
| `lexer.by` | ✅ | ❌ | `class` fields → `_call` linker error |
| `llvm.by` | ✅ | ❌ | Necesita `-lLLVM` al linkear |
| `mir.by` | ✅ | ❌ | Runtime funcs sin link |
| `keywords.by` | ❌ | ❌ | No imports entre archivos Byle |
| `parser.by` | ❌ | ❌ | Solo expresiones, sin statements |
| `checker.by` | ❌ | ❌ | Antes: borrow; ahora: `and` parser issue |
| `driver.by` | ❌ | ❌ | Antes: borrow |
| `chars.by` | ❌ | ❌ | `^str.get/set` → `_call` |
| `lexer_test.by` | ✅ | ❌ | Struct ABI (igual que lexer) |

---

## Bugs Restantes en el Compilador Rust (bloquean ky2c.by)

| # | Bug | Síntoma | Dónde arreglar |
|---|-----|---------|----------------|
| 1 | **Strings en while-loop pierden contenido** | `s = s + x` dentro de `while` produce `s.len()` correcto pero `s` vacío | `byc_mir/ssa.rs` o `byc_backend/codegen.rs` |
| 2 | **`!=` en strings compara punteros** | `s[i] != prefix[i]` siempre true aunque chars iguales | `byc_mir/lower.rs` (no hay overload de `!=` para str) |
| 3 | **`^str.get/set`** genera `_call` | `pos.set(pos.get() + 1)` → `_call` linker error | `byc_mir/lower.rs` (mutable method dispatch) |
| 4 | **Structs con campo `str`** no se pueden leer | `tok.text` produce output vacío | `byc_backend/codegen.rs` (struct field str ABI) |
| 5 | **`^{str}` lista** da valores basura | `items.push("hello"); items.get(0)` → garbage | `byc_backend/codegen.rs` (str→i64 cast) |
| 6 | **`extern fn` retorna `str`** produce `_call` | `by_read_str()` espera `ptr`, Byle espera `str` | ABI mismatch entre runtime y type system |

**El bug #1 es el bloqueador principal de ky2c.by.**

---

## Pipeline Real para Zero Rust

```
HOY: .by ─▶ Rust parser ─▶ Rust types ─▶ Rust MIR ─▶ Rust LLVM ─▶ binary
              (byc_frontend) (byc_semantic) (byc_mir) (byc_backend)
              + Rust runtime (byc_runtime)

FALTAN 6 BUGS DEL COMPILADOR RUST
↓

PASO 1: Arreglar bug #1 (while-loop strings)
  → ky2c.by puede correr sin acumular en loops
  → genera C correcto para input de una línea

PASO 2: Arreglar bugs #2-#6 
  → ky2c.by puede compilar archivos completos
  → genera C correcto para cualquier input

PASO 3: ky2c.by se compila a sí mismo
  → cat ky2c.by | by run ky2c.by > ky2c.c
  → clang ky2c.c -o ky2c
  → ./ky2c < ky2c.by > ky2c2.c
  → cmp ky2c.c ky2c2.c  # IDÉNTICO → KENO! Punto fijo alcanzado

PASO 4: Portar runtime a Byle
  → Reemplazar byc_runtime/ (Rust) con runtime.by (Byle)
  → Usar @link "c" + extern fn para syscalls

PASO 5: Cero Rust
  → by (Rust) compila ky2c.by → ky2c binario
  → ky2c compila by_compiler.by → by binario
  → by (Byle) compila su propio código fuente
```

## Próximo Paso Inmediato

**Arreglar bug #1**: strings en while-loop pierden contenido.

Es un bug en `byc_mir/ssa.rs` o `byc_backend/codegen.rs`. La string se crea correctamente (length correcto) pero el pointer a datos se pierde. Probablemente el SSA mem2reg o la promoción de alloca está matando el buffer de la string.

Crear repro mínimo y arreglarlo. Eso destraba ky2c.by.
