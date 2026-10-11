# Project Configuration (`by.toml`)

> `by.toml` es el archivo de configuración de cualquier proyecto Byle, sea CLI,
> servidor, web, librería, o lo que sea. Todo proyecto Byle tiene uno.

---

## 1. Ejemplo completo

```toml
[project]
name = "mi-app"
version = "0.1.0"
edition = "2024"
description = "Una aplicación de ejemplo"
authors = ["Tu Nombre <email@ejemplo.com>"]
license = "MIT"

[entry]
main = "src/main.by"        # archivo de entrada (default: "main.by")
# main = "."                # también funciona: todo en la raíz
# main = "app.by"           # o un archivo específico

[compiler]
optimization = "O2"          # "O0", "O1", "O2", "O3"
target = "native"            # "native", "arm64", "x86_64", "wasm", "freestanding"
release = false              # default false

[output]
dir = "target"               # directorio de salida (default: "target")
bin = "mi-app"               # nombre del binario (default: name del proyecto)

[dependencies]
http = "0.1.0"               # package del registro
json = { path = "../json" }  # package local
sqlite = "0.1.0"
```

## 2. Sección `[project]`

Datos del proyecto.

| Campo | Tipo | Default | Descripción |
|-------|------|---------|-------------|
| `name` | string | **(requerido)** | Nombre del proyecto |
| `version` | string | `"0.1.0"` | Versión semver |
| `edition` | string | `"2024"` | Edición del lenguaje |
| `description` | string | `""` | Descripción corta |
| `authors` | string[] | `[]` | Lista de autores |
| `license` | string | `""` | Licencia (MIT, Apache, etc.) |

## 3. Sección `[entry]`

Configura el punto de entrada del proyecto.

| Campo | Tipo | Default | Descripción |
|-------|------|---------|-------------|
| `main` | string | `"main.by"` | Archivo principal |

`main` puede ser:
- `"src/main.by"` — ruta específica dentro del proyecto
- `"app.by"` — archivo en la raíz
- `"."` — todo el directorio raíz como entrada (busca `*.by`)
- `"src/"` — todos los archivos `.by` en `src/`

Si `main` no existe, `by` busca en orden:
1. `main.by` (raíz)
2. `src/main.by`
3. `app.by`
4. `index.by`
5. `src/index.by`

**Nota**: `by run`, `by build`, `by check` pueden recibir un archivo específico
como argumento (ej: `by run index.by`), ignorando `[entry]`. Si no se pasa
archivo, se usa `[entry]`.

## 4. Sección `[compiler]`

Opciones del compilador.

| Campo | Tipo | Default | Descripción |
|-------|------|---------|-------------|
| `optimization` | string | `"O2"` | Nivel de optimización LLVM |
| `target` | string | `"native"` | Target triple |
| `release` | bool | `false` | Build release (O3 + SSA) |

`optimization` puede ser: `"O0"`, `"O1"`, `"O2"`, `"O3"`.

`target` puede ser:
- `"native"` — detecta automáticamente
- `"arm64"` — macOS ARM / Linux ARM64
- `"x86_64"` — Linux/macOS Intel
- `"wasm"` — WebAssembly
- `"freestanding"` — kernel, sin runtime

## 5. Sección `[output]`

Controla dónde y cómo se genera el binario.

| Campo | Tipo | Default | Descripción |
|-------|------|---------|-------------|
| `dir` | string | `"target"` | Directorio de salida |
| `bin` | string | `name` del proyecto | Nombre del ejecutable |

Estructura de salida:
```
target/
├── debug/
│   └── <bin>           # ejecutable (debug)
├── release/
│   └── <bin>           # ejecutable (release)
├── <bin>.ll            # LLVM IR
└── <bin>.o             # object file
```

## 6. Sección `[dependencies]`

Packages que el proyecto necesita.

```toml
[dependencies]
http = "0.1.0"               # del registro oficial
json = { path = "../json" }  # ruta local
sqlite = { git = "..." }     # desde git (futuro)
```

| Formato | Descripción |
|---------|-------------|
| `"0.1.0"` | Versión del registro oficial (GitHub Pages) |
| `{ path = "..." }` | Ruta local al package |
| `{ git = "..." }` | URL de git (futuro) |

Las dependencias se resuelven en este orden:
1. `packages/<name>/` (desarrollo local)
2. `std/<name>/` (instalado vía `by add`)
3. `~/.by/cache/<name>/` (caché global)

## 7. Ejemplos por tipo de proyecto

### CLI tool
```toml
[project]
name = "mi-cli"
version = "0.1.0"
edition = "2024"

[entry]
main = "main.by"

[dependencies]
json = "0.1.0"
```

### Web server
```toml
[project]
name = "mi-api"
version = "0.1.0"
edition = "2024"

[entry]
main = "src/server.by"

[compiler]
optimization = "O2"
target = "native"

[dependencies]
http = "0.1.0"
json = "0.1.0"
sqlite = "0.1.0"
```

### Library
```toml
[project]
name = "mi-lib"
version = "0.1.0"
edition = "2024"

[dependencies]
json = "0.1.0"
```

### Kernel (freestanding)
```toml
[project]
name = "mi-kernel"
version = "0.1.0"
edition = "2024"

[entry]
main = "kernel.by"

[compiler]
target = "freestanding"
optimization = "O3"
```

## 8. Orden de resolución

Cuando ejecutas `by run`, `by build`, o `by check`:

1. Si se pasa un archivo explícito (`by run index.by`): **usa ese archivo**
2. Si no: busca en orden: `main` de `[entry]` → `main.by` → `src/main.by` → `app.by` → `index.by` → `src/index.by`
3. Si encuentra el archivo: lo compila y ejecuta
4. Si no: error "no entry point found"

## 9. Comportamiento `by run` sin `fn main()`

Byle permite ejecutar scripts sin `fn main()` explícita. El compilador
auto-genera un `main` que ejecuta el código en orden secuencial:

```byle
# index.by — sin fn main()
name = input("ingresa tu nombre: ")
print("hola " + name)
```

Esto es equivalente a:
```byle
fn main():
    name = input("ingresa tu nombre: ")
    print("hola " + name)
```

**Reglas:**
- Si el archivo NO tiene ninguna función `fn main()`: se auto-genera
- Si YA tiene `fn main()`: se usa esa
- Si tiene funciones pero ninguna se llama `main`: error (debe definir `main` o no tener ninguna función)
