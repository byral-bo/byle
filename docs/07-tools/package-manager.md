# Package Manager (Technical Reference)

> Documentation tecnica del gestor de packages.
> Para uso basico, ver `02-getting-started/package-manager.md`.

## Comandos

| Comando | Description |
|---------|-------------|
| `by add <pkg>` | Agregar dependency |
| `by remove <pkg>` | Eliminar dependency |
| `by install` | Instalar dependencies del proyecto |
| `by publish` | Publicar package en registry |

## Formato del package

```
package-name/
├── by.toml # metadata del package
└── src/
 └── lib.by # code source
```

## Registry URL

```
https://IT-KYNERA.github.io/BYLE/docs
```

## See also

- `02-getting-started/package-manager.md` — Guide de uso
- `08-ecosystem/registry.md` — Specification del registry
