# Editor Support

> Byle supports any editor via the Language Server Protocol (`by lsp`).

## LSP

The Byle compiler includes an LSP server. Any editor with LSP support (VS Code, Neovim, Helix, Emacs, etc.) can use it:

```bash
by lsp
```

## Vim / Neovim

```vim
autocmd BufRead,BufNewFile *.by set filetype=python
```

## Helix

```toml
[[language]]
name = "byle"
scope = "source.by"
file-typis = ["by"]
indent = { tab-width = 4, unit = " " }
```

## VS Code

VS Code extension is available in the [byle-vscode](https://github.com/IT-KYNERA/byle-vscode) repository.

## See also

- `language-server.md` — LSP features
