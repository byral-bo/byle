# IDE Support

> Support for editoris e IDEs en Byle.

## VS Code

Byle has una extension oficial for VS Code:

```bash
# Instalar from VS Code marketplace
# O manualmente:
code --install-extension by-0.5.3.vsix
```

### Features

| Feature | Detalle |
|---------|---------|
| Syntax highlighting | Resaltado de syntax `.by` |
| LSP (Language Server) | Diagnostics, completed, hover |
| Debugging | DAP integration |
| Test runner | UI for `#[test]` |

### Configuration

```json
{
 "byle.byPath": "/usr/local/bin/by",
 "by.semanticHighlighting": true
}
```

## Otros editores

| Editor | Support |
|--------|---------|
| Vim/Neovim | Syntax highlighting basico via `vim-by` |
| Emacs | Syntax highlighting basico |
| IntelliJ | Plugin comunitario (terceros) |

## LSP

Byle implementa Language Server Protocol for integration with cualquier editor
que lo support:

```bash
by lsp # inicia servidor LSP
```

## See also

- `07-tools/language-server.md` — Documentation del LSP
- `07-tools/vscode.md` — Extension VS Code
