<div align="center">

# Byle

**A compiled, statically-typed language for backend systems and CLI tools.**

Readable like Python · Typed like Rust · Simple like Go · **Fast like C**

[![License: MIT](https://img.shields.io/badge/license-MIT-6C3FC5?style=for-the-badge)](LICENSE)
[![Release](https://img.shields.io/badge/release-v0.8.9-6C3FC5?style=for-the-badge)](https://github.com/kyal-tech/byle/releases/latest)
[![Platform](https://img.shields.io/badge/platform-macOS%20ARM/x64%20%7C%20Linux%20ARM/x64%20%7C%20Windows%20x64-6C3FC5?style=for-the-badge)](#install)
[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-6C3FC5?style=for-the-badge)](https://www.rust-lang.org)

</div>

---

## Install

### Compiler (`by`)

**macOS / Linux**

```bash
curl -fsSL https://raw.githubusercontent.com/kyal-tech/byle/main/scripts/install.sh | sh
```

> Requires macOS 12+ (ARM64) or any modern Linux (x64 / ARM64).

**Windows**

```powershell
iwr -Uri "https://raw.githubusercontent.com/kyal-tech/byle/main/scripts/install.ps1" | iex
```

> Requires Windows 10+ (x64). Run PowerShell as Administrator.

**Direct downloads**

| Platform | Arch | Download |
| :--- | :--- | :--- |
| **macOS** | ARM64 | [by-macos-arm64.tar.gz](https://github.com/kyal-tech/byle/releases/download/v0.8.9/by-macos-arm64.tar.gz) |
| **Linux** | ARM64 | [by-linux-arm64.tar.gz](https://github.com/kyal-tech/byle/releases/download/v0.8.9/by-linux-arm64.tar.gz) |
| **Linux** | x64 | [by-linux-x64.tar.gz](https://github.com/kyal-tech/byle/releases/download/v0.8.9/by-linux-x64.tar.gz) |
| **Windows** | x64 | [by-windows-x64.zip](https://github.com/kyal-tech/byle/releases/download/v0.8.9/by-windows-x64.zip) |

> macOS Intel (x64) is no longer supported. Use Apple Silicon (ARM64).

Editor integration lives in the separate `byle-vscode` project.

---

## Why Byle?

Byle compiles directly to **native machine code** via LLVM. You get the ergonomics of a high-level language with the performance of C — without garbage collection pauses, without a heavyweight runtime, and without giving up type safety.

- **Native speed** — compiles to machine code, benchmarks within **1.0–1.3× of C**
- **Statically typed** — deep type system: `?` optional, `!` fallible, `^` mutable, `&` borrow
- **Batteries included** — threads, channels, mutexes, async, TCP, HTTP, JSON, regex, crypto
- **Packages** — install and reuse libs with `by add`
- **Predictable** — no GC, deterministic ownership, low memory footprint

---

## Quick Start

```bash
# Create a project and run it
by new myapp && cd myapp
by run
```

Or compile a single file straight to a native binary:

```bash
echo 'print("Hello from Byle!")' > hello.by
by run hello.by
by build hello.by && ./hello   # native binary
```

---

## Hello, Byle

```byle
print("Hello, World!")
```

### Variables — immutable by default, mutable with `^`

```byle
name = "Byle"          # immutable (default)
count: ^i32 = 0        # mutable
count += 1
```

### Fallible results with `!` — no more exceptions

```byle
fn parse(s: &str) i32!:
    n = int(s)?
    if n < 0: return error("negative")
    n

x: i32! = parse("42")
y = x!                 # propagates the error automatically
```

### Possibilities with `?` — no more null checks

```byle
fn find_user(id: i32) User?:
    ...
user = find_user(1)?
if user == nil: ...
```

### Orthogonal types

```byle
x: i32            # primitive
x: i32?           # optional
x: i32!           # fallible (Result)
x: ^i32           # mutable
x: &str           # borrow
x: ^&[i32]!       # mutable borrow of a list, may error
```

### Collections

```byle
items = [1, 2, 3]                    # list [i32]
nums = set{1, 2, 3}                  # set <i32>
dict = {"name": "byle", "ver": 1}    # dict {str: i32}
q = queue{1, 2, 3}                   # queue <i32>
s = stack{"a", "b"}                  # stack <str>
```

---

## Performance

Byle compiles to native machine code and sits right next to C. Benchmarks on an **Apple M5**, release mode, median of 15 runs. Bar length = relative speed (longer = faster); full length = the fastest language in that test.

```
Fibonacci (500M iters)          (lower time = faster)
C     |████████████████████████████|  118 ms
C++   |███████████████████████████|  121 ms
Rust  |███████████████████████████|  122 ms
Byle  |██████████████████████████|  128 ms
```

```
Prime Sieve (3M)                (lower time = faster)
C     |███████████████████████████|  8.5 ms
C++   |████████████████████████████|  8.3 ms
Rust  |███████████████████████████|  8.6 ms
Byle  |██████████████████████|     10.6 ms
```

```
String Concat (500k)            (lower time = faster)
C     |██████|                     8.2 ms
C++   |███████|                    8.1 ms
Rust  |████████████████████████████|  1.9 ms
Byle  |█████|                      10.3 ms
```

```
MatMul (100×100)                (lower time = faster)
C     |████████████████████████████|  6.6 ms
C++   |███████████████████████████|  6.8 ms
Rust  |██████████████████████████|  7.2 ms
Byle  |████████████████████████████|  6.7 ms
```

**Byle is on par with C** — within **1.0–1.3×** across every benchmark, and competitive with Rust on integer-heavy workloads.

> Benchmark history: [`docs/11-project/benchmarks.md`](docs/11-project/benchmarks.md)

---

## Packages

The standard library ships with the language. Install extra libraries with `by add`:

```bash
by add http        # HTTP client/server
by add postgres    # PostgreSQL
by add sqlite      # SQLite
by add env         # .env loader
```

| Package | Description |
| :------- | :---------- |
| [`http`](docs/packages/http.md) | HTTP client and server |
| [`postgres`](docs/packages/postgres.md) | PostgreSQL driver |
| [`sqlite`](docs/packages/sqlite.md) | SQLite driver |
| [`env`](docs/packages/env.md) | .env loader |

```byle
use std.http
use std.json
use http.server.*
```

---

## Commands

```bash
by new <project>      # create new project
by run <file.by>      # compile and run
by build <file.by>    # compile to native binary
by check <file.by>    # type-check only
by parse <file.by>    # dump AST
by mir <file.by>      # dump MIR
by fmt <file.by>      # format source
by test               # run project tests
by add <package>      # install a package
```

---

## Documentation

| Resource | Location |
| :------- | :------- |
| Language Syntax | `docs/03-language/syntax/` |
| Type System | `docs/09-specification/type-system.md` |
| Collections | `docs/03-language/syntax/collections.md` |
| Modules & Imports | `docs/03-language/syntax/modules.md` |
| Syntax Reference | `docs/15-byle-syntax-reference.md` |
| VS Code Extension | Separate `byle-vscode` project |
| Benchmarks | `docs/11-project/benchmarks.md` |
| Roadmap | `docs/11-project/roadmap.md` |

---

## Build from Source

Requires **LLVM 18** and **Rust 1.81+**.

```bash
# Linux (Debian/Ubuntu)
sudo apt install llvm-18-dev libpolly-18-dev libzstd-dev

# macOS
brew install llvm@18
export LLVM_SYS_181_PREFIX=$(brew --prefix llvm@18)

# Windows (PowerShell as Admin)
choco install llvm --version=18.1.8
$env:LLVM_SYS_181_PREFIX = "C:\Program Files\LLVM"

# Build
git clone https://github.com/kyal-tech/byle.git
cd byle
cargo build --release --bin by
```

---

## License

[MIT](LICENSE) — Copyright (c) 2026 [Kynera](https://github.com/IT-KYNERA)
