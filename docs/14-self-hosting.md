# Self-hosting — Portear Byle de Rust a Byle

## ¿Qué significa "Byle corre Byle"?

Hoy `by` (el compilador) está escrito en **Rust** (10+ crates, ~50k líneas).
`libbyc_runtime.a` (el runtime) está escrito en **Rust** (~2k líneas).

**Byle se compila a sí mismo** cuando el compilador `by` está escrito en Byle, y compila su propio código fuente.

---

## ¿Qué se puede pasar a Byle AHORA?

Sin cambiar nada del compilador, usando solo `extern fn` + `@link` (FFI a C) + lógica pura:

### ✅ Runtime (YA se puede)

Cada función del runtime Rust se reescribe en Byle. Las syscalls van por FFI.

```byle
# Antes (Rust):
// byc_runtime/src/memory.rs
pub extern "C" fn by_alloc(size: i64) -> *mut u8 {
    unsafe { libc::malloc(size as usize) as *mut u8 }
}

# Después (Byle):
# runtime/memory.by
@link "c"
extern fn malloc(size: i64) ptr
extern fn free(ptr)

fn by_alloc(size: i64) ptr:
    return malloc(size)

fn by_free(p: ptr):
    free(p)
```

Es **exactamente lo mismo** que los packages (`http`, `json`, `sqlite`) — 100% Byle con FFI a C.

| Componente del runtime | Líneas (Rust) | Portar a Byle |
|------------------------|:------------:|:-------------:|
| `memory.rs` (alloc/free) | 80 | ✅ Ahora |
| `string.rs` (str ops) | 200 | ✅ Ahora |
| `list.rs` (List[T]) | 250 | ✅ Ahora |
| `dict.rs` (Dict[K,V]) | 180 | ✅ Ahora |
| `io.rs` (print, read) | 150 | ✅ Ahora |
| `net.rs` (TCP, WS) | 400 | ✅ Ahora (FFI a libc) |
| `channel.rs` (Chan[T]) | 100 | ✅ Ahora (FFI a pthread) |
| `thread.rs` (spawn) | 80 | ✅ Ahora (FFI a pthread) |
| `decimal.rs`, `datetime.rs`, `uuid.rs` | 300 | ✅ Ahora |
| `json.rs` | 200 | ✅ Ahora |
| **Total** | **~2,000** | **✅ Ahora (~2-3 semanas)** |

### ✅ Build System / CLI (YA se puede)

`by build`, `by run`, `by check` son solo lógica + subprocesos:

```byle
# Antes (Rust):
fn cmd_build(args: &[String]) {
    let source = std::fs::read_to_string(&file).unwrap();
    let mir = Pipeline::mir_source(&source, &file).unwrap();
    let codegen = Codegen::new(&context, "module");
    codegen.compile_with_ssa(&mir.module).unwrap();
    linker.link(&[&obj], output, ...).unwrap();
}

# Después (Byle):
# cli/build.by
extern fn system(cmd: &str) i32
extern fn read_file(path: &str) str
extern fn write_file(path: &str, content: &[u8])

fn cmd_build(source: str, file: str, output: str):
    # El parser, type checker, MIR, LLVM codegen
    # se llaman vía FFI a librerías Byle compiladas
    ast = parse(source)
    hir = lower(ast)
    sem = check_types(hir)
    mir = lower_to_mir(sem)
    llvm_ir = codegen(mir)
    write_file(output + ".ll", llvm_ir)
    system("llc " + output + ".ll -o " + output + ".o")
    system("clang " + output + ".o -o " + output)
```

**Nota:** Todo EXCEPTO el codegen LLVM es lógica pura (strings, listas, match, if/for). Eso funciona HOY.

### ✅ Parser + Type Checker + MIR (YA se puede)

| Componente | Es lógica pura? | Se puede portar HOY? |
|-----------|:--------------:|:--------------------:|
| Lexer (caracteres → tokens) | ✅ Sí | ✅ Ahora |
| Parser (tokens → AST) | ✅ Sí | ✅ Ahora |
| HIR (AST desugar) | ✅ Sí | ✅ Ahora |
| Semantic (tipos, scopes) | ✅ Sí | ✅ Ahora |
| MIR (lower + optimize) | ✅ Sí | ✅ Ahora |
| SSA (mem2reg, phi) | ✅ Sí | ✅ Ahora |

Todos estos son algoritmos puros. No necesitan nada especial de Byle. Strings, enums, pattern matching, listas, if/for — Byle tiene todo eso.

---

## ¿Qué NECESITA una feature nueva de Byle?

### 🟡 LLVM Codegen (requiere FFI bindings a LLVM C API)

El codegen traduce MIR → LLVM IR. Hoy usa `inkwell` (Rust bindings a LLVM).

Para portarlo a Byle, necesitamos `extern fn` para la **C API de LLVM**:

```byle
# llvm.by — Bindings a LLVM C API
@link "llvm"
extern fn LLVMModuleCreateWithName(name: &str) ptr
extern fn LLVMAddFunction(mod: ptr, name: &str, ty: ptr) ptr
extern fn LLVMAppendBasicBlock(fn_val: ptr, name: &str) ptr
extern fn LLVMBuildRetVoid(builder: ptr)
extern fn LLVMBuildAdd(builder: ptr, lhs: ptr, rhs: ptr, name: &str) ptr
# ... ~200 funciones más

fn compile_module(mir: MirModule) -> ptr:
    mod = LLVMModuleCreateWithName("module")
    for func in mir.functions:
        fn_ty = make_fn_type(func.params, func.return_type)
        fn_val = LLVMAddFunction(mod, func.name, fn_ty)
        bb = LLVMAppendBasicBlock(fn_val, "entry")
        builder = LLVMCreateBuilder()
        LLVMPositionBuilderAtEnd(builder, bb)
        for inst in func.body:
            compile_inst(builder, inst)
    return mod
```

**La C API de LLVM existe y es estable.** Es el mismo camino que `http` con `libcurl`, `sqlite` con `libsqlite3`. Solo que ~200 funciones en vez de ~20.

### ❌ Inline Assembly

`asm("cli")` — necesario para el kernel, NO para el compilador. No bloquea self-hosting.

---

## ¿Qué se DEJA en Rust (por ahora)?

| Componente | Razón | Se puede pasar después? |
|-----------|-------|:----------------------:|
| **LLVM codegen** | ~200 funciones FFI a LLVM C API | ✅ Sí, cuando tengamos los bindings |
| **Optimizaciones** (LTO, inlining) | LLVM las maneja | ✅ Sí, vía bindings |
| **Linker** (clang/ld) | Solo llama a subprocesos | ✅ Sí, con `system()` FFI |

---

## Mapa de migración

```
HOY:  Byle .by ─▶ Rust parser ─▶ Rust types ─▶ Rust MIR ─▶ Rust LLVM ─▶ binary
                     (byc_frontend)  (byc_semantic)  (byc_mir)  (byc_backend)
                     + Rust runtime (byc_runtime)

FASE 1 (2-3 sem):  Byle .by ─▶ Byle parser ─▶ Byle types ─▶ Byle MIR ─▶ Rust LLVM ─▶ binary
                                  ↑               ↑             ↑
                              Portar a Byle   Portar a Byle  Portar a Byle
                                  │               │             │
                                  └── Todo esto corre HOY en Byle ──┘

                     Byle runtime (portado de Rust)
                     ↑── Esto también corre HOY en Byle

FASE 2 (4-6 sem):  Byle .by ─▶ Byle parser ─▶ ... ─▶ Byle MIR ─▶ Byle LLVM bindings ─▶ binary
                                                                     (FFI a libLLVM)
                                                  Byle runtime

FASE 3 (bootstrap): El Byle viejo (Rust) compila el nuevo Byle (Byle)
                     El nuevo Byle (Byle) se compila a sí mismo
                     🎉 BYLE CORRE BYLE
```

## ¿Esto bloquea KYOS?

**NO.** KYOS puede construirse con el compilador Rust perfectamente. El self-hosting es un objetivo a largo plazo que no bloquea absolutamente nada de KYOS.

El flujo sería:

```
PARALELO:
  ├── Workstream A (KYOS Services) → Byle user-space → compila con Rust by
  ├── Workstream B (KYOS Desktop)  → BYUI nativo    → compila con Rust by
  ├── Workstream C (KYOS Kernel)   → kernel Byle     → compila con Rust by + freestanding
  └── Workstream D (Self-hosting)  → portear Rust a Byle → compilador Byle en Byle
```

Workstream D corre en segundo plano y no bloquea a nadie.

---

## Referencia de sintaxis

Para portar el compilador, usar `docs/15-byle-syntax-reference.md` como referencia rápida de toda la sintaxis de Byle con ejemplos funcionales.
