# Build

> Compilar proyectos Byle with CLI `by`.

## Compilar un file

```bash
by build file.by # compile (debug)
by build --release file.by # compile with optimizaciones
by build file.by -o output # especificar name de output
```

## Compilar y execute

```bash
by run file.by # compila y ejecuta
by run --release file.by # compila optimizado y ejecuta
```

## Type-check rapido

```bash
by check file.by # solo type checking, without code objeto
```

## Proyectos

```bash
by build # compile proyecto current
by build --release # compile en release
by check # type-check del proyecto
```

## Flags

| Flag | Description |
|------|-------------|
| `--release` | Compilation optimizada (O3 + LTO) |
| `-o` | Nombre del binary de output |
| `--target` | Target triple (ej: `wasm32`) |

## See also

- `installation.md` — Instalar Byle
- `first-program.md` — Primer program
- `06-compiler/pipeline.md` — Pipeline de compilation
