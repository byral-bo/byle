# CLI Reference

## Commands

| Command | Description |
|---------|-------------|
| `by build <file>` | Compile to native binary |
| `by run <file>` | Compile and execute |
| `by check <file>` | Type-check only |
| `by parse <file>` | Parse and dump AST |
| `by mir <file>` | Dump MIR |
| `by fmt [file]` | Format source code |
| `by test` | Run tests |
| `by new <project>` | Create new project |
| `by add <dep>` | Add dependency |
| `by remove <dep>` | Remove dependency |
| `by update` | Update lockfile |
| `by publish` | Publish package |
| `by login` | Login to registry |
| `by lsp` | Start language server |
| `by completions <shell>` | Generate completions |

## Flags

| Flag | Description |
|------|-------------|
| `--release` | Optimized build (SSA + O3) |
| `--emit-ir` | Dump LLVM IR |
