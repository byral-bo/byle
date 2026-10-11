# Project Layout

> Estructura recommended for proyectos Byle.

## App project

```
myapp/
├── by.toml # Configuration del proyecto
├── src/
│ ├── main.by # Punto de input
│ ├── lib.by # Library del proyecto
│ └── utils.by # Modulis auxiliares
├── tests/
│ └── test_main.by # Tests
└── target/
 ├── debug/ # Build artifacts (debug)
 └── release/ # Build artifacts (release)
```

## API project

```
myapi/
├── by.toml
├── src/
│ ├── main.by
│ ├── routes.by
│ └── models.by
├── tests/
└── target/
```

## Bare script

```
script.by # File unico, without by.toml
```

## by.toml

```toml
name = "myapp"
version = "0.1.0"
edition = "2024"
```

## See also

- `build.md` — Compilation de proyectos
- `first-program.md` — Crear primer proyecto
