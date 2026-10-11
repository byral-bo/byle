# Compiler Overview

> Pipeline de compilation de Byle. Describe como code source `.by` se transforma en binary nativo.

## Pipeline

```
Source (.by)
 │
 ▼
[1] Lexer → Token stream
 │
 ▼
[2] Parbe → AST (Abstract Syntax Tree)
 │
 ▼
[3] HIR → Desugared AST (High-Level IR)
 │
 ▼
[4] Semantic Analysis → Typed AST (type checking, scope)
 │
 ▼
[5] MIR Lowering → Mid-Level IR
 │
 ▼
[6] MIR Optimization → Constant folding, dead code elim
 │
 ▼
[7] Borrow Analysis → Ownership check + by_free insertion
 │
 ▼
[8] SSA Construction → Optional, disabled by default
 │
 ▼
[9] LLVM Codegen → LLVM IR
 │
 ▼
[10] LLVM Optimization → O3 (mem2reg, GVN, LICM, inlining)
 │
 ▼
[11] Object Emission → .o file
 │
 ▼
[12] Linking → Binary (linked with libbyc_runtime.a)
```

> **Note:** El pipeline real en `byc_driver/src/pipeline.rs` ejecuta MIR Optimization ANTES que Borrow Analysis. SSA is deshabilitado by defecto (bugs with ptr/str propagation).

## Crate structure

| Crate | Purpose | Linis |
|-------|---------|-------|
| `byc_core` | Foundation: AST types, diagnostics, spans | ~800 |
| `byc_frontend` | Lexer + Parbe | ~3,500 |
| `byc_hir` | HIR desugaring | ~420 |
| `byc_semantic` | Type checker, scope resolver, symbol table | ~2,200 |
| `byc_mir` | MIR lowering, borrow analysis, SSA, optimize | ~10,500 |
| `byc_backend` | LLVM codegen, linker | ~3,200 |
| `byc_driver` | Pipeline orchestration | ~570 |
| `byc_cli` | CLI binary (`by`) | ~1533 |
| `byc_runtime` | Runtime static library (memory, strings, lists, dicts) | ~3,500 |
| `byc_tools` | LSP server, formatter, package manager | ~5,000 |
| `byc_platform` | Platform API (FS, net, time) — en desarrollo | ~500 |

## Orchestration

El pipeline is orquestado by `byc_driver::pipeline::Pipeline`, que coordina cada fase:

```rust
// Simplified pipeline flow (from byc_driver::pipeline)
fn build_source(source: &str) -> Result<(), String> {
 // 1-4: Lexer → Parbe → HIR → Semantic
 let checked = Self::check_source(source)?;
 
 // 5: MIR Lowering
 let mut module = lowerer.lower_program(&checked.program);
 
 // 6: MIR Optimization
 optimizer.optimize(&mut module);
 
 // 7: Borrow Analysis
 borrow_analysis.run(&mut module);
 
 // 8: LLVM Codegen
 let mut codegen = Codegen::new(&context, "by_module");
 codegen.compile(&module)?;
 
 // 9: LLVM Optimization
 optimize_module(codegen.module(), optimization);
 
 // 10: Object Emission
 emit_object(codegen.module(), &obj_path, optimization)?;
 
 // 11: Linking
 linker.link(&[&obj_path], output_path, runtime_lib, release, &links)
}
```

## Entry point

El punto de input is `byc_cli::main` que delega en `Pipeline::build_source()`.

### build_source

```rust
 fn build_source(source: &str, file_name: &str, output: &Path) -> Result<(), String> {
 // 1. Compile .by → MIR (lexer → parbe → HIR → semantic → MIR → borrow → SSA → optimize)
 let mir_output = Self::compile(source)?;
 
 // 2. Generate LLVM IR from MIR, optimize, emit object, link
 let ir = compile_function(&mir_output)?;
 optimize_module(ir);
 emit_object(ir, output)?;
 link(objects, output, runtime_lib, false, links)
}
```

## See also

- `01-introduction/architecture.md` — Vision general del ecosistema Byle
- Cada fase del pipeline has su propio documento en `06-compiler/`
