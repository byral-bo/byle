# Package Manager

> Gestor de paquetis de Byle: `by add`, `by remove`, `by install`.

## Comandos

### Agregar dependency

```bash
by add http # agrega http from registry
by add http@1.0.0 # version especifica
```

### Instalar dependencies

```bash
by install # instala todas dependencies de by.lock
```

### Eliminar dependency

```bash
by remove http # elimina http
```

### Publicar package

```bash
by publish # publica en registry local
```

## Registry

Los paquetis se distribuyen via GitHub Pages:

```text
https://IT-KYNERA.github.io/BYLE/docs
```

## Paquetis oficiales

| Package | Description |
|---------|-------------|
| `http` | Cliente y servidor HTTP |
| `json` | Parseo y serializacion JSON |
| `sqlite` | Base de data SQLite |

## by.lock

El file `by.lock` se genera automaticamente al instalar dependencies.
No se must modificar manualmente.

## See also

- `build.md` — Compilation
- `project-layout.md` — Estructura de proyectos
- `08-ecosystem/` — Documentation del ecosistema
