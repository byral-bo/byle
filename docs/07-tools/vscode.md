# VS Code Extension

**Status:** External project — [`byle-vscode`](https://github.com/IT-KYNERA/byle-vscode)

## Install

**macOS / Linux:**
```bash
git clone https://github.com/IT-KYNERA/byle-vscode.git
```

**Windows (PowerShell):**
```powershell
iwr -Uri "https://raw.githubusercontent.com/IT-KYNERA/byle-vscode/main/install.ps1" | iex
```

Or install from the VS Code Marketplace: search "Byle Language Support".

The extension source and release assets do not belong to this compiler repository.

## Features

- **Syntax highlighting** — TextMate grammar for `.by` files
- **LSP integration** — diagnostics, autocomplete, go-to-def, hover, find refs, rename
- **Inlay hints** — inferred types shown inline
- **Code lens** — "Run test" button above `#[test]` functions
- **Task provider** — auto-discovers `.by` files for run/build/check
- **Debug adapter** — DAP-based debugging with breakpoints, stack traces, scopes
- **Test UI** — VS Code TestController integration for `#[test]` functions
- **Problems panel** — compiler diagnostics shown as problems
- **Color theme** — "Byle Pastel" dark theme
- **Snippets** — code snippets for common patterns

## Configuration

| Setting | Description |
|---------|-------------|
| `byle.byPath` | path to `by` binary (auto-detected) |
| `byle.semanticHighlighting` | Enable experimental semantic highlighting |

## Binary search order

1. `byle.byPath` setting
2. PATH environment
3. macOS/Linux: `~/.by/bin/by`, `~/.cargo/bin/by`, `/usr/local/bin/by`, `/opt/homebrew/bin/by`
4. Windows: `%USERPROFILE%\.by\bin\by.exe`
5. `where` (Windows) / `which` (Unix) command
