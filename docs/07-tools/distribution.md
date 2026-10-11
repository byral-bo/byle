# Distribution — Multi-Platform Install

**Status:** Release procedure
**Date:** 2026-07-09
**Documentaci&oacute;n relacionada:**
- [`docs/10-design/rfc/0002-ui-architecture.md`](../10-design/rfc/0002-ui-architecture.md) — UI framework roadmap
- [`docs/02-getting-started/installation.md`](../02-getting-started/installation.md) — Instalaci&oacute;n desde c&oacute;digo fuente

---

## 1. Objetivo

Que cualquier persona pueda instalar Byle en **cualquier sistema operativo y arquitectura**
con **un solo comando**, y que al hacerlo reciba:

- El compilador `by` (o `by.exe`) para su plataforma
- El runtime `libbyc_runtime.a` para su plataforma
- Configuraci&oacute;n autom&aacute;tica del PATH

Sin necesidad de tener Rust, LLVM, ni ning&uacute;n otro requisito previo instalado.

### 1.1 Plataformas objetivo

| Plataforma | Arquitectura | Triplete LLVM | Bundle |
|------------|-------------|---------------|--------|
| macOS | ARM64 (Apple Silicon) | `aarch64-apple-darwin` | `by-macos-arm64.tar.gz` |
| Linux | ARM64 | `aarch64-unknown-linux-gnu` | `by-linux-arm64.tar.gz` |
| Linux | x86_64 | `x86_64-unknown-linux-gnu` | `by-linux-x64.tar.gz` |
| Windows | x86_64 | `x86_64-pc-windows-gnu` | `by-windows-x64.zip` |

---

## 2. Experiencia de instalaci&oacute;n

### 2.1 macOS / Linux

El usuario ejecuta un solo comando:

```bash
curl -fsSL https://raw.githubusercontent.com/kyal-tech/byle/main/scripts/install.sh | sh
```

El script:

1. Detecta SO y arquitectura (`uname -s` + `uname -m`)
2. Descarga el bundle correcto desde GitHub Releases
3. Extrae `by` y `libbyc_runtime.a`
4. Instala en `~/.by/bin/by` + `~/.by/lib/libbyc_runtime.a`
5. Si hay permisos de sudo, pregunta y puede instalar en `/usr/local/`
6. Agrega `~/.by/bin` al PATH en `.zshrc` / `.bashrc`
7. Muestra mensaje de &eacute;xito

Si el usuario ya tiene Byle, el script detecta la versi&oacute;n actual y pregunta si desea actualizar.

### 2.2 Windows

El usuario ejecuta en PowerShell:

```powershell
iwr -Uri "https://raw.githubusercontent.com/kyal-tech/byle/main/scripts/install.ps1" | iex
```

El script:

1. Detecta arquitectura (x64 o ARM64)
2. Descarga el ZIP correcto desde GitHub Releases
3. Extrae `by.exe` y `libbyc_runtime.a`
4. Instala en `%USERPROFILE%\.by\bin\by.exe` + `%USERPROFILE%\.by\lib\libbyc_runtime.a`
5. Agrega `%USERPROFILE%\.by\bin` al PATH de usuario
6. Muestra mensaje de &eacute;xito

### 2.3 Descarga directa (avanzado)

Usuarios avanzados pueden descargar manualmente desde GitHub Releases:

```
https://github.com/kyal-tech/byle/releases/download/v0.8.9/by-macos-arm64.tar.gz
https://github.com/kyal-tech/byle/releases/download/v0.8.9/by-windows-x64.zip
...
```

---

## 3. Estructura de los bundles

### 3.1 macOS / Linux (.tar.gz)

```
by-macos-arm64.tar.gz
  by                    # Binario compilado (executable)
  libbyc_runtime.a      # Runtime est&aacute;tico
  LICENSE               # Licencia MIT
```

### 3.2 Windows (.zip)

```
by-windows-x64.zip
  by.exe                # Binario compilado (executable)
  libbyc_runtime.a      # Runtime est&aacute;tico
  LICENSE               # Licencia MIT
```

---

## 4. Script de instalaci&oacute;n: `install.sh`

### 4.1 Detecci&oacute;n de plataforma

```bash
detect_platform() {
    local os arch

    case "$(uname -s)" in
        Darwin) os="macos" ;;
        Linux)  os="linux" ;;
        *)      echo "Unsupported OS: $(uname -s)"; exit 1 ;;
    esac

    case "$(uname -m)" in
        arm64|aarch64) arch="arm64" ;;
        x86_64|amd64)  arch="x64" ;;
        *)      echo "Unsupported arch: $(uname -m)"; exit 1 ;;
    esac

    echo "${os}-${arch}"
}
```

### 4.2 Flujo completo

```bash
#!/bin/bash
set -eu

REPO="kyal-tech/byle"
VERSION="v0.8.9"
PLATFORM=$(detect_platform)

# Preguntar si requiere sudo (solo para /usr/local/)
if [ -w /usr/local/bin ]; then
    INSTALL_PREFIX="/usr/local"
    USE_SUDO=false
else
    echo "Install to /usr/local requires sudo."
    echo "Install to ~/.by/ instead? [Y/n]"
    read -r answer
    if [ "$answer" != "n" ]; then
        INSTALL_PREFIX="$HOME/.by"
        USE_SUDO=false
    else
        echo "Please run with sudo or install manually."
        exit 1
    fi
fi

# Descargar bundle
BUNDLE="by-${PLATFORM}.tar.gz"
URL="https://github.com/$REPO/releases/download/$VERSION/$BUNDLE"

echo "Downloading Byle $VERSION for $PLATFORM..."
curl -fsSL "$URL" -o "/tmp/$BUNDLE"

# Extraer
tar xzf "/tmp/$BUNDLE" -C /tmp/

# Instalar
mkdir -p "$INSTALL_PREFIX/bin" "$INSTALL_PREFIX/lib/by"
cp /tmp/by "$INSTALL_PREFIX/bin/by"
cp /tmp/libbyc_runtime.a "$INSTALL_PREFIX/lib/by/libbyc_runtime.a"
chmod +x "$INSTALL_PREFIX/bin/by"

# Agregar al PATH
# ...

echo "✅ Byle $VERSION installed to $INSTALL_PREFIX"
```

### 4.3 Parámetros adicionales

| Variable | Efecto |
|----------|--------|
| `BY_VERSION=v0.8.9` | Instalar versión específica |
| `BY_PREFIX=/custom/path` | Directorio de instalación |
| `BY_NO_MODIFY_PATH=1` | No modificar PATH automáticamente |

---

## 5. Script de instalación: `install.ps1`

```powershell
# install.ps1
param(
    [string]$Version = "v0.8.9",
    [string]$Prefix = "$env:USERPROFILE\.by"
)

$arch = if ([Environment]::Is64BitOperatingSystem) { "x64" } else { "arm64" }
$bundle = "by-windows-$arch.zip"
$url = "https://github.com/kyal-tech/byle/releases/download/$Version/$bundle"

Write-Host "Downloading Byle $Version for Windows..."
Invoke-WebRequest -Uri $url -OutFile "$env:TEMP\$bundle"

Expand-Archive "$env:TEMP\$bundle" -DestinationPath $Prefix

# Add to PATH
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$Prefix\bin*") {
    [Environment]::SetEnvironmentVariable(
        "PATH", "$userPath;$Prefix\bin", "User"
    )
}

Write-Host "✅ Byle $Version installed to $Prefix"
```

---

## 6. Build/Release script: `scripts/build-release.sh`

### 6.1 Cross-compilación

`scripts/build-release.sh` usa `Cargo.lock` y distingue tres casos:

- En el host nativo compila `by` y `byc_runtime`, crea el bundle y verifica su SHA-256.
- En targets no nativos puede compilar el runtime con `cargo-zigbuild`, pero omite el bundle completo porque `by` depende de las librerías LLVM nativas del host.
- Un fallo real de compilación termina el script con error; los targets no instalados se reportan como omitidos.

La forma recomendada de generar releases completos es el workflow nativo de GitHub Actions (`.github/workflows/release.yml`). El script local se ejecuta así:

```bash
./scripts/build-release.sh
```

Los bundles publicados contienen `by`, `libbyc_runtime.a`, `LICENSE` y su
archivo `.sha256`. El instalador rechaza bundles que no tengan checksum.

### 6.2 Toolchains necesarias

| Target | Toolchain | Instalación |
|--------|-----------|-------------|
| `aarch64-apple-darwin` | Nativo (macOS ARM64) | — |
| `x86_64-apple-darwin` | Rust target | `rustup target add x86_64-apple-darwin` |
| `x86_64-unknown-linux-gnu` | Rust target + Zig | `rustup target add ...` + `cargo install cargo-zigbuild` + Zig |
| `aarch64-unknown-linux-gnu` | Rust target + Zig | `rustup target add ...` + `cargo install cargo-zigbuild` + Zig |
| `x86_64-pc-windows-gnu` | Rust target + Zig | `rustup target add ...` + `cargo install cargo-zigbuild` + Zig |

---

## 7. Target triples del compilador

El soporte de target explícito está implementado en el backend, pipeline y CLI.
Esta sección documenta el diseño vigente y las limitaciones de distribución;
los bundles completos no nativos se construyen en CI con runners nativos.

### 7.1 Flag `--target` en `by build`

El compilador debe aceptar un target triple opcional:

```bash
by build app.by                          # usa target del host
by build --target wasm32 app.by          # WebAssembly
by build --target x86_64-pc-windows-gnu  # Windows cross
```

### 7.2 codegen.rs — Target triple configurable

Implementación por defecto:

```rust
let triple = inkwell::targets::TargetMachine::get_default_triple();
module.set_triple(&triple);
```

Cuando se proporciona un target explícito se usa:

```rust
let triple = self.target_triple.clone()
    .unwrap_or_else(|| inkwell::targets::TargetMachine::get_default_triple());
module.set_triple(&triple);
```

### 7.3 linker.rs — Linker por target

El linker selecciona la estrategia según el target triple:

| Triplete | Linker |
|----------|--------|
| `aarch64-apple-darwin` | `clang -arch arm64` |
| `x86_64-apple-darwin` | `clang -arch x86_64` |
| `x86_64-unknown-linux-gnu` | `x86_64-linux-gnu-gcc` |
| `aarch64-unknown-linux-gnu` | `aarch64-linux-gnu-gcc` |
| `x86_64-pc-windows-gnu` | `x86_64-w64-mingw32-gcc` |
| `wasm32-unknown-unknown` | `wasm-ld` |

### 7.4 linker.rs — Búsqueda de runtime

El compilador busca el runtime del host en las ubicaciones instaladas (`/usr/local/lib`,
`~/.by/lib` y rutas relativas al binario). Los bundles de CI son autocontenidos;
no se instala una colección de runtimes cruzados en el host. Para añadir soporte
de runtimes por target en una futura instalación multi-target, la ruta prevista es:

```
~/.by/lib/targets/<triple>/libbyc_runtime.a
```

---

## 8. Estructura de directorios instalada

```
~/.by/                          # (o /usr/local/lib/by/)
  bin/
    by                          # Compilador
  lib/
    libbyc_runtime.a            # Runtime de la plataforma
```

---

## 9. Seguridad

| Medida | Descripción |
|--------|-------------|
| **HTTPS** | Todas las descargas via HTTPS desde GitHub |
| **Checksums** | Cada release incluye SHA-256 del bundle |
| **Firma macOS** | Firma ad-hoc del binario y dylibs cuando aplica |
| **Verificación** | `install.sh` exige y verifica el checksum antes de instalar |
| **Minimal** | Solo se descarga lo necesario para la plataforma |

---

## 10. Mantenimiento

### Por release

1. Ejecutar CI para generar los cuatro bundles oficiales (Linux ARM64/x64, macOS ARM64 y Windows x64)
2. Verificar checksums y contenido de cada bundle
3. Subir bundles a GitHub Releases
4. (Opcional) Actualizar install.sh si cambia la URL base

### Nueva plataforma

Para agregar un nuevo target (ej: Linux RISC-V):

1. Agregar el target a `TARGETS` en `build-release.sh`
2. Agregar el triplete al mapa de bundles
3. Agregar detección en `install.sh` y `install.ps1`

---

## 11. Referencias

- [`docs/10-design/rfc/0002-ui-architecture.md`](../10-design/rfc/0002-ui-architecture.md) — UI framework roadmap
- [`docs/02-getting-started/installation.md`](../02-getting-started/installation.md) — Instalación desde fuente
- [`docs/07-tools/compiler-cli.md`](./compiler-cli.md) — CLI del compilador
- [`docs/07-tools/package-manager.md`](./package-manager.md) — Package manager
