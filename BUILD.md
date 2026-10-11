# Building Byle

## Requirements

- **Rust** 1.81+ (`rustup install stable`)
- **LLVM 18** runtime libraries
- **make**, **gcc**/**clang** (for linking)

### Install LLVM 18

```bash
# macOS
brew install llvm@18
export LLVM_SYS_181_PREFIX=$(brew --prefix llvm@18)

# Linux (Debian/Ubuntu)
sudo apt install llvm-18-dev libpolly-18-dev libzstd-dev

# Windows (PowerShell as Admin)
choco install llvm --version=18.1.8
$env:LLVM_SYS_181_PREFIX = "C:\Program Files\LLVM"
```

## Build the Compiler

```bash
git clone https://github.com/kyal-tech/byle.git
cd byle

# Release build (recommended)
cargo build --release --bin by --locked

# Debug build
cargo build --bin by --locked
```

## Quick Test

```bash
# Run a .by file
echo 'print("Hello from Byle!")' > hello.by
./target/release/by run hello.by

# Type-check only
./target/release/by check hello.by

# Build UI app (.byx)
./target/release/by build app.byx

# Build for desktop
./target/release/by build app.byx desktop

# Build for iOS
./target/release/by build app.byx --target ios
```

## Run Tests

```bash
# All workspace tests
cargo test --workspace --locked

# Syntax tests
for f in tests/syntax/*.by; do
    ./target/release/by run "$f" && echo "PASS: $f" || echo "FAIL: $f"
done

# Standard library and end-to-end checks
./scripts/test-by.sh ./target/release/by

# Dependency security audit
cargo install cargo-audit --version 0.22.2 --locked
cargo audit --deny warnings

# Gate completo antes de release o commit
./scripts/pre-release-check.sh
```

### Nota sobre macOS/Homebrew

En algunos entornos macOS con LLVM instalado mediante Homebrew, `cargo` puede
mostrar un aviso del linker sobre una ruta alternativa de `libunwind`. Si la
compilación termina correctamente y `cargo clippy --workspace --locked -- -D warnings`
no reporta errores, ese aviso pertenece al toolchain del sistema y no al
código de Byle. CI valida el build en macOS y Linux por separado.

## Install

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/kyal-tech/byle/main/scripts/install.sh | sh

# Windows (PowerShell)
iwr -Uri "https://raw.githubusercontent.com/kyal-tech/byle/main/scripts/install.ps1" | iex
```

## Editor integration

The VS Code extension is maintained separately in `byle-vscode`.
