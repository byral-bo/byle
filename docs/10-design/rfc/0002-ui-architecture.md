# RFC-0002: Byle UI Framework

**Status:** Implementado (FASIS A-C) — Actualizado 2026-07-11
**Date:** 2026-07-09 (creado)
**Documentaci&oacute;n relacionada:**
- [`docs/10-design/rfc/0003-ui-translation.md`](0003-ui-translation.md) — Traducci&oacute;n multi-target (JS/WASM/nativo)
- [`docs/10-design/rfc/0005-ui-rearchitecture-plan.md`](0005-ui-rearchitecture-plan.md) — Plan de readaptaci&oacute;n multi-plataforma
- [`docs/03-language/syntax/ui-syntax.md`](../../03-language/syntax/ui-syntax.md) — sintaxis .byx
- [`docs/07-tools/distribution.md`](../../07-tools/distribution.md) — distribuci&oacute;n multi-plataforma (Fase 0)

---

## 1. Visi&oacute;n General

Byle UI es un sistema de UI declarativo que permite escribir una sola vez la interfaz
en `.byx` y desplegarla en **cualquier plataforma**:

| Plataforma | Target | Pipeline |
|------------|--------|----------|
| **Web** | `wasm32-unknown-unknown` | Byle → LLVM → WASM → DOM |
| **Windows** | `x86_64-pc-windows-gnu` | Byle → LLVM → Win32/Skia |
| **macOS** | `aarch64-apple-darwin` | Byle → LLVM → Cocoa/Skia |
| **Linux** | `x86_64-unknown-linux-gnu` | Byle → LLVM → X11/Wayland/Skia |
| **Android** | `aarch64-linux-android` | Byle → LLVM → Android NDK |
| **iOS** | `aarch64-apple-ios` | Byle → LLVM → UIKit/SwiftUI |
| **Custom OS** | `x86_64-unknown-byleos` | Byle → LLVM → Byle Graphics Stack |

El principio fundamental: **un solo lenguaje, un solo compilador, m&uacute;ltiples backends**.

### 1.1 Filosof&iacute;a de Dise&ntilde;o

| Principio | Significado |
|-----------|-------------|
| **Declarativo** | El qu&eacute;, no el c&oacute;mo. El framework optimiza el renderizado. |
| **Sin JS** | Todo el c&oacute;digo es Byle. No hay transpilaci&oacute;n a JavaScript. |
| **FFI nativo** | Cada plataforma expone sus APIs v&iacute;a `extern fn` + `@link`. |
| **Un solo parser** | .byx se transforma a AST de Byle. El mismo pipeline sem&aacute;ntico + MIR + LLVM. |
| **Runtime en Byle** | Los componentes UI se escriben en Byle puro (como http, json, sqlite). |

---

## 2. C&oacute;mo se compila .byx

El archivo `.byx` no es un lenguaje nuevo. Es **az&uacute;car sint&aacute;ctico** que el compilador
transforma a Byle est&aacute;ndar antes de pasarlo al pipeline existente.

### 2.1 Pipeline de compilaci&oacute;n

```
.byx (XML-like)
  │
  ▼
┌──────────────────────┐
│  .byx Parser         │  ← NUEVO: transforma XML a AST Byle
│  (frontend/byx.rs)   │
└──────────┬───────────┘
           ▼
┌──────────────────────┐
│  AST Byle            │  ← Existente: mismo AST
│  (byc_core/ast.rs)   │
└──────────┬───────────┘
           ▼
┌──────────────────────┐
│  Semantic + HIR      │  ← Existente
│  + MIR + SSA         │
└──────────┬───────────┘
           ▼
┌──────────────────────┐
│  LLVM Codegen        │  ← Existente: set target triple
│  (byc_backend)       │
└──────────┬───────────┘
           ▼
    .wasm / .exe / app  ← Seg&uacute;n target
```

### 2.2 Transformaci&oacute;n XML → AST

```byx
<button text="Login" click=@login />
```

se transforma a:

```byle
button(text: "Login", click: login)
```

El parser .byx debe soportar:

| Sintaxis .byx | Equivalente Byle |
|---------------|------------------|
| `<Componente />` | `componente()` |
| `<Comp attr=@val />` | `comp(attr: val)` |
| `<Comp attr="str" />` | `comp(attr: "str")` |
| `<Comp>hijo</Comp>` | `comp(hijo)` |
| `@expr` | `expr` |
| `@(bloque)` | Bloque Byle embebido |
| `@if:` / `@for:` | Control flow |
| `@match:` | Pattern matching |
| `style<X> Name:` | Struct de estilo |
| `theme Name:` | Struct de tema |

### 2.3 Extensi&oacute;n de archivo

- `.byx` = vista (componente o p&aacute;gina)
- `.by` = l&oacute;gica opcional (el mismo componente puede tener ambos)

El module resolver debe buscar `.byx` adem&aacute;s de `.by`:

```
use Login.view
  → busca Login.by (no existe)
  → busca Login.byx (existe) → lo parsea
```

---

## 3. Roadmap de Implementaci&oacute;n

### Fase 0 — Base (ahora)

**Objetivo:** Que Byle se instale en cualquier plataforma y el compilador soporte target triples.

| # | Tarea | Dependencia |
|---|-------|-------------|
| 0.1 | Instalador multi-plataforma (install.sh + install.ps1) | — |
| 0.2 | Flag `--target` en `by build` | — |
| 0.3 | Linker seleccionable por target (no por host) | 0.2 |
| 0.4 | Runtime cross-compilado para cada target | 0.1 |
| 0.5 | Directorio de runtimes: `~/.by/lib/targets/<triple>/` | 0.4 |

Documento asociado: [`docs/07-tools/distribution.md`](../../07-tools/distribution.md)

---

### Fase 1 — Parser .byx

**Objetivo:** Poder escribir `.byx` y compilarlo a nativo.

| # | Tarea | Dependencia |
|---|-------|-------------|
| 1.1 | Registrar `.byx` como extensi&oacute;n conocida | — |
| 1.2 | Parser XML → AST de Byle (nuevo crate: `byc_ui`) | — |
| 1.3 | Module resolver busca `.byx` adem&aacute;s de `.by` | 1.1 |
| 1.4 | `by build login.byx` produce ejecutable nativo | 1.2, 1.3 |
| 1.5 | Soporte de fragmentos `@(...)` con c&oacute;digo Byle | 1.2 |
| 1.6 | Soporte de `@if`, `@for`, `@match` en .byx | 1.5 |
| 1.7 | Soporte de `style<>`, `layout<>`, `tpl<>`, `theme` | 1.5 |
| 1.8 | Soporte de slots y componentes hijos | 1.4 |

---

### Fase 2 — WebAssembly

**Objetivo:** Compilar .byx a WASM y correr en el navegador.

| # | Tarea | Dependencia |
|---|-------|-------------|
| 2.1 | Soportar target `wasm32-unknown-unknown` en codegen | 0.2 |
| 2.2 | Compilar `byc_runtime` a WASM | — |
| 2.3 | Linker WASM: `wasm-ld` en linker.rs | 0.3 |
| 2.4 | Runtime DOM: bindings Byle → JS (`extern fn`) | — |
| 2.5 | `by new webapp` — template con HTML shell | 2.1–2.4 |
| 2.6 | Eventos del DOM (click, input, change) en Byle | 2.4 |
| 2.7 | Two-way binding en WASM | 2.6 |
| 2.8 | Optimizaci&oacute;n: tama&ntilde;o del .wasm, lazy loading | 2.1 |

**Detalle t&eacute;cnico WASM:**

```
LLVM genera .wasm directamente (wasm32-unknown-unknown)
  │
  ▼
Runtime Byle compilado a WASM (memoria, strings, lists)
  │
  ▼
DOM bindings via imports WASM:
  (import "dom" "createElement" (func ...))
  (import "dom" "addEventListener" (func ...))
  │
  ▼
Browser carga el WASM. Byle llama a funciones JS
para manipular el DOM. Sin JS engorroso, todo desde Byle.
```

---

### Fase 3 — UI Runtime Nativo (Skia)

**Objetivo:** Componentes UI nativos que renderizan igual en Windows, macOS y Linux.

| # | Tarea | Dependencia |
|---|-------|-------------|
| 3.1 | Crear `packages/ui/` con componentes base en Byle | — |
| 3.2 | FFI a Skia (`@link "libskia"`) para canvas 2D | — |
| 3.3 | Ventana nativa: Win32, Cocoa, X11 v&iacute;a FFI | — |
| 3.4 | Sistema de layout (flexbox-like) | 3.1 |
| 3.5 | Componentes: View, Text, Button, Image | 3.2, 3.4 |
| 3.6 | Componentes: TextField, Scroll, List, Form | 3.5 |
| 3.7 | Componentes: Navigation, Tab, Drawer, Dialog | 3.6 |
| 3.8 | Sistema de estilos y temas (from `style<>`/`theme`) | 3.1 |
| 3.9 | Animaciones y transiciones | 3.8 |
| 3.10 | `by new app` — template multiplataforma | 3.5 |

---

### Fase 4 — Mobile (Android + iOS)

**Objetivo:** Compilar .byx a apps nativas de m&oacute;vil.

| # | Tarea | Dependencia |
|---|-------|-------------|
| 4.1 | Target `aarch64-linux-android` (Android NDK) | 0.2 |
| 4.2 | Target `aarch64-apple-ios` (Xcode toolchain) | 0.2 |
| 4.3 | Runtime Android: SurfaceView + touch events | 3.2 |
| 4.4 | Runtime iOS: Metal + UIKit bridge | 3.2 |
| 4.5 | Componentes nativos mobile (BottomBar, Swipe, etc.) | 3.5 |
| 4.6 | `by new mobile` — template Android + iOS | 4.1–4.5 |

---

### Fase 5 — Custom OS

**Objetivo:** Byle como lenguaje de sistema para un OS basado en Linux.

| # | Tarea | Dependencia |
|---|-------|-------------|
| 5.1 | Byle runtime sin libc (freestanding) | — |
| 5.2 | Byle graphics stack en lugar de X11/Wayland | 3.2 |
| 5.3 | Kernel Linux + init en Byle | 5.1 |
| 5.4 | Sistema de ventanas en Byle | 5.2 |
| 5.5 | Drivers de dispositivo en Byle (FFI al kernel) | 5.1 |
| 5.6 | `by build --target byle-os` | 0.2 |

---

## 4. Arquitectura del Runtime UI

El runtime UI sigue el mismo patr&oacute;n que los paquetes existentes (http, json, sqlite):

```
packages/ui/
  src/
    lib.by             # Entry point, re-exports
    components/
      view.by          # <view>
      text.by          # <text>
      button.by        # <button>
      text_field.by    # <text_field>
      ...
    layout/
      flex.by          # Flexbox layout engine
      grid.by          # Grid layout
    style/
      theme.by         # Sistema de temas
      animation.by     # Animaciones
    platform/
      web.by           # DOM bindings (Fase 2)
      desktop.by       # Skia bindings (Fase 3)
      mobile.by        # Mobile bindings (Fase 4)
    extern/
      skia.by          # @link "libskia"
      dom.by           # @link a funciones JS (WASM)
      win32.by         # @link "user32"
      cocoa.by         # @link a frameworks macOS
      x11.by           # @link "libX11"
```

Cada componente UI en Byle:

```byle
# packages/ui/src/components/button.by

extern fn skia_draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, radius: f32)
extern fn skia_draw_text(x: f32, y: f32, text: &str)

final class Button:
    text: str
    on_click: ^&(fn ())

    fn render(this, x: f32, y: f32, w: f32, h: f32):
        skia_draw_rounded_rect(x, y, w, h, 4.0)
        skia_draw_text(x + 10, y + 20, this.text)
```

---

## 5. Decisiones T&eacute;cnicas Vinculantes

### 5.1 Flag `--target`

`by build` debe aceptar `--target` para especificar la plataforma de salida:

```bash
by build app.by                           # nativo (host)
by build --target wasm32 app.by           # WebAssembly
by build --target x86_64-unknown-linux-gnu app.by  # Linux x64
```

Esto implica:
- `codegen.rs`: usar el triple del flag, no `get_default_triple()`
- `linker.rs`: seleccionar linker seg&uacute;n el triple, no `cfg!(target_os)`

### 5.2 Resoluci&oacute;n de m&oacute;dulos

El resolver debe priorizar `.by` sobre `.byx` si ambos existen:

```
use Login.view
  → Login.by existe? → usarlo (.by tiene prioridad)
  → Login.byx existe? → parsearlo como .byx
  → Error: no encontrado
```

### 5.3 Directorio de runtimes

```
~/.by/
  bin/by
  lib/
    libbyc_runtime.a                       # runtime host
    targets/
      aarch64-apple-darwin/                # macOS ARM
      x86_64-apple-darwin/                 # macOS Intel
      x86_64-unknown-linux-gnu/            # Linux x64
      aarch64-unknown-linux-gnu/           # Linux ARM
      x86_64-pc-windows-gnu/               # Windows x64
      wasm32-unknown-unknown/              # WebAssembly
      ...
```

Cada subdirectorio contiene:
```
libbyc_runtime.a              # Runtime base compilado para ese target
libbyc_ui_runtime.a           # Runtime UI (Fase 3+)
```

### 5.4 Componentes nativos del sistema

Los componentes predefinidos (App, View, Text, Button, etc.) se implementan
como paquetes Byle, no como built-ins del compilador. El compilador solo
provee el pipeline de compilaci&oacute;n y el FFI.

---

## 6. Lo que NO hacer (anti-patrones)

| Anti-patr&oacute;n | Riesgo | Alternativa |
|-------------------|--------|-------------|
| Backend especializado para un solo target | El c&oacute;digo se vuelve insostenible cuando llegan m&aacute;s targets | LLVM ya maneja todos; solo configura el triple |
| Linker hardcodeado por OS | No se puede cross-compilar | Seleccionar linker por target (din&aacute;mico) |
| Parser de .byx separado del de .by | Duplicaci&oacute;n de l&oacute;gica sem&aacute;ntica | .byx → AST Byle → pipeline existente |
| Runtime UI monol&iacute;tico | Dif&iacute;cil de mantener y portar | Componentes modulares (un archivo por componente) |
| Bindings JS espec&iacute;ficos de un browser | No funciona en todos los browsers | DOM est&aacute;ndar W3C |
| Dependencia de npm/Node.js para web | Contradice la filosof&iacute;a "sin JS" | WASM puro, sin Node.js |
| Compilaci&oacute;n JIT o interpreter para UI | P&eacute;rdida de rendimiento vs nativo | LLVM O3 + AOT compilation |

---

## 7. Glosario

| T&eacute;rmino | Significado |
|---------------|-------------|
| .byx | Archivo de vista Byle UI (XML-like) |
| Target triple | Identificador de plataforma LLVM (ej: `wasm32-unknown-unknown`) |
| WASM | WebAssembly, formato binario para navegadores |
| FFI | Foreign Function Interface, mecanismo para llamar C desde Byle |
| Skia | Biblioteca de gr&aacute;ficos 2D multiplataforma (usada por Chrome, Flutter) |
| Shell HTML | `index.html` que carga un m&oacute;dulo WASM |
| Componente | Unidad de UI reutilizable en .byx |
| Slot | Lugar reservado para contenido hijo en un componente |
| Binding | Conexi&oacute;n bidireccional entre variable y UI |

---

## 8. Referencias

- [`docs/03-language/syntax/ui-syntax.md`](../../03-language/syntax/ui-syntax.md) — Sintaxis completa de .byx
- [`docs/07-tools/distribution.md`](../../07-tools/distribution.md) — Distribuci&oacute;n multi-plataforma (Fase 0)
- [`docs/03-language/ffi/abi.md`](../../03-language/ffi/abi.md) — FFI extern fn + @link
- LLVM WebAssembly target: `wasm32-unknown-unknown`
- Skia Graphics Library: https://skia.org
