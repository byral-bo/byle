# WASM — WebAssembly Target

**Version:** 1.0 
**Status:** Specification

---

## 1. What is it?

Byle compila a WebAssembly via LLVM (`wasm32-unknown-unknown`).
Esto allows execute code Byle en navegador.

```
.by → LLVM IR → wasm32 → .wasm → navegador
```

---

## 2. Compilation

```bash
# Compilar a WASM
by build --target wasm32-unknown-unknown app.by

# Optimizado for size
by build --target wasm32-unknown-unknown -O3 app.by
```

### Limitacionis del target WASM

| Recurso | Disponible |
|---------|-----------|
| CPU (calculos) | ✅ |
| Memory lineal | ✅ |
| Math (f32/f64) | ✅ |
| Strings | ✅ |
| file system | ❌ (no there is FS en WASM) |
| Networking | ❌ (no there is sockets) |
| Threads | ❌ |
| Console/print | ❌ (requiere JS glue) |

---

## 3. web — Browbe API bindings

El package `web` expone APIs del navegador a Byle compilado a WASM.

```byle
use web.{document, console, fetch}
```

### DOM

```byle
use web.document

div = document.get_element_by_id("app")
div.text_content = "Hola from Byle!"

# Crear elements
btn = document.create_element("button")
btn.text_content = "Click me"
btn.onclick = (event):
 console.log("clicked!")
div.append_child(btn)
```

### Fetch (HTTP from browser)

```byle
use web.{fetch, response}

ris = fetch("/api/users")
if res.ok:
 users = res.json()
 for ube in users:
 print(user["name"])
```

### Console

```byle
use web.console

console.log("hello from Byle")
console.error("something went wrong")
```

### Canvas 2D

```byle
use web.{document, canvas}

canvas = document.get_element_by_id("game")
ctx = canvas.getContext("2d")

ctx.fill_style = "red"
ctx.fill_rect(10, 10, 100, 50)

ctx.fill_style = "blue"
ctx.font = "20px sans-serif"
ctx.fill_text("Byle!", 20, 80)
```

---

## 4. Estructura del package web

```
packages/web/
├── by.toml
└── src/
 ├── lib.by # Exportacionis principales
 ├── dom.by # DOM API (get_element_by_id, create_element, etc.)
 ├── fetch.by # Fetch API
 ├── canvas.by # Canvas 2D
 ├── console.by # Console API
 └── events.by # Event listeners
```

### Implementation

Las APIs del navegador se exponen via `extern fn` with `@link "js"`:

```byle
@link "js"

extern fn js_getElementById(id: ptr) ptr
extern fn js_setTextContent( : ptr, text: ptr)
extern fn js_createElement(tag: ptr) ptr
extern fn js_appendChild(parent: ptr, child: ptr)
extern fn js_fetch(url: ptr) ptr
```

Un runtime de JS glue traduce llamadas a APIs realis del navegador.

---

## 5. Flujo de trabajo

```bash
# 1. Compilar Byle a WASM
by build --target wasm32-unknown-unknown app.by -o app.wasm

# 2. Crear HTML with JS glue
cat > index.html << 'HTML'
<!DOCTYPE html>
<script src="runtime.js"></script>
<script>
 const wasm = await WebAssembly.instantiateStreaming(
 fetch("app.wasm"), { js: KyJsGlue }
 );
 wasm.instance.exports.main();
</script>
HTML
```

---

## 6. Plan de implementation

| Fase | Description | Status |
|------|-------------|--------|
| 1 | Compilation WASM via LLVM | ✅ (LLVM target existe) |
| 2 | JS glue runtime basico | 🔜 |
| 3 | web: console + DOM basico | 🔜 |
| 4 | web: fetch, canvas, events | 🔜 |
| 5 | Optimization de size WASM | 🔜 |
| 6 | Reactive UI framework (JSX-like) | 📅 |
