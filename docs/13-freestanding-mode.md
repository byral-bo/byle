# Freestanding Mode — Roadmap para kernel en Byle

> **Estado actual:** Parcialmente implementado
> **Documento técnico:** Qué falta para compilar kernels bare-metal con Byle

---

## 1. Estado actual

```
by build freestanding kernel.by   →   entry: _start
                                     ✅ no wrapper main()
                                     ✅ nombre de funciones intacto
                                     ⚠️ runtime libbyc_runtime.a linkeado
                                     ❌ depende de libc (malloc, write, pthread)
                                     ❌ linker script default del host
                                     ❌ macOS frameworks linkeados
```

El modo freestanding actual NO sirve para bare-metal. Solo evita el wrapper de `main`. El binario generado depende completamente del sistema operativo host.

---

## 2. Lo que falta para un kernel REAL

### 2.1 Runtime bare-metal (`byc_runtime_baremetal`)

| Función | Implementación actual | Necesario para bare-metal |
|---------|----------------------|---------------------------|
| `by_alloc` | `malloc()` de libc | Bump allocator desde `_end` |
| `by_free` | `free()` de libc | No-op (no hay free en bump) |
| `by_print/by_println` | `write()` syscall | UART/serial out |
| `by_concat` | `malloc + memcpy` | Bump allocator |
| `by_strlen` | `strlen()` de libc | Implementación inline |
| `by_memcpy/memset` | `memcpy/memset` de libc | Implementación inline |
| threading | `pthread` | No aplica en kernel single-core |
| channels | `pthread_mutex` | No aplica |
| TCP/networking | sockets de libc | Drivers de red propios |

### 2.2 Linker flags

| Flag | Estado | Por qué |
|------|--------|---------|
| `-nostdlib` | ❌ | Elimina dependencia de libc |
| `-nostartfiles` | ❌ | Elimina CRT (crt0, crti, crtn) |
| `-ffreestanding` | ❌ | Indica al compilador que es freestanding |
| `-T linker.ld` | ❌ | Linker script personalizado |
| `-e _start` | ✅ | Entry point correcto |

### 2.3 Cross-compilation

| Target | Estado | Notas |
|--------|--------|-------|
| `x86_64-unknown-none` | ❌ | Kernel x86_64 puro |
| `aarch64-unknown-none` | ❌ | Kernel ARM64 puro |
| `x86_64-unknown-elf` | ❌ | ELF freestanding |
| Host triple | ✅ | Solo compila para tu máquina |

### 2.4 Inline assembly

```
Instrucciones necesarias para x86_64:
- cli / sti         → habilitar/deshabilitar interrupciones
- lgdt              → cargar GDT
- lidt              → cargar IDT
- ltr               → cargar TSS
- mov cr3, rax      → cambiar tabla de páginas
- hlt               → halt CPU
- in / out          → I/O ports (UART, PIC, etc.)
- wrmsr / rdmsr     → MSR registers
- int 0x80 / syscall → system calls
```

Estado: **❌ No implementado.** Byle no tiene sintaxis para `asm()`.

### 2.5 Memory management

| Componente | Estado |
|------------|--------|
| Page table manipulation (CR3) | ❌ |
| Bump allocator | ❌ |
| Virtual memory mapping | ❌ |
| Heap allocator (kmalloc/kfree) | ❌ |

### 2.6 Interrupt handling

| Componente | Estado |
|------------|--------|
| IDT setup | ❌ |
| IRQ handlers | ❌ |
| Exception handlers (page fault, GPF, etc.) | ❌ |
| PIC/APIC configuration | ❌ |
| Timer interrupt (PIT/HPET) | ❌ |

### 2.7 Bootloader protocol

| Protocolo | Estado |
|-----------|--------|
| Multiboot2 header (GRUB) | ❌ |
| Limine protocol | ❌ |
| STIVALE2 protocol | ❌ |
| Device tree (ARM) | ❌ |
| UEFI application | ❌ |

---

## 3. Plan de implementación por fases

### Fase A: Hosted freestanding (1-2 semanas)
_Objetivo: Poder compilar un kernel que corre bajo Linux usando QEMU_

- [ ] Agregar `-nostdlib -nostartfiles -ffreestanding` al linker en modo freestanding
- [ ] Crear `byc_runtime_baremetal/` con allocator simple (bump)
- [ ] Implementar `by_print` vía UART (port I/O)
- [ ] Implementar `by_memcpy/memset/strlen` inline
- [ ] Agregar flag `--target x86_64-unknown-none` a la CLI
- [ ] Crear linker script mínimo (`kernel.ld`)
- [ ] Test con QEMU (kernel que escribe "OK" al puerto serie)

### Fase B: Kernel mínimo (3-4 semanas)
_Objetivo: Kernel que bootea en QEMU con GDT, IDT, página_

- [ ] Agregar sintaxis `asm()` a Byle (inline assembly)
- [ ] Implementar GDT setup
- [ ] Implementar IDT setup
- [ ] Implementar page table management
- [ ] Implementar timer interrupt handler
- [ ] Implementar keyboard interrupt handler
- [ ] Implementar bump allocator como `by_alloc` bare-metal

### Fase C: Runtime bare-metal (4-6 semanas)
_Objetivo: Que el runtime de Byle funcione sin libc_

- [ ] Portar `by_concat` a bump allocator
- [ ] Portar `by_list_*` a bump allocator
- [ ] Portar `by_dict_*` a bump allocator
- [ ] Implementar `by_str_to_i64` sin libc
- [ ] Implementar `by_now` sin syscalls (RTC/HPET)
- [ ] Eliminar dependencia de `pthread` en modo bare-metal

### Fase D: Usermode + syscalls (8-12 semanas)
_Objetivo: Poder ejecutar programas Byle en el kernel_

- [ ] Implementar TSS + ring 3
- [ ] Implementar syscall handler
- [ ] Implementar scheduler básico (round-robin)
- [ ] Implementar ELF loader
- [ ] Implementar `fork/exec` primitivos
- [ ] Compilar `by` como programa de usuario en KYOS

---

## 4. Dependencias con el compilador

### 4.1 Lo que el compilador YA soporta (útil para kernel)

| Feature | Por qué es útil |
|---------|-----------------|
| `extern fn` + `@link` | Llamar a hardware via FFI (ports, MMIO) |
| `ptr` type | Manipular memoria arbitraria |
| `unsafe` blocks | Operaciones sin borrow checking |
| Structs con `#[repr(C)]` | Mapping de estructuras de hardware |
| `--target freestanding` | Entry point `_start` sin wrapper |
| Enums con payload | Tagged unions para mensajes |
| Match exhaustivo | Manejo de interrupciones seguro |

### 4.2 Lo que el compilador NECESITA para kernel

| Feature | Prioridad | Esfuerzo estimado |
|---------|:---------:|:-----------------:|
| Inline assembly `asm("cli")` | 🔴 Alta | 1-2 semanas |
| Sección attributes `section(".multiboot")` | 🔴 Alta | 1 semana |
| `#[repr(packed)]` para structs hardware | 🟡 Media | 1 semana |
| Linker script support en CLI | 🟡 Media | 3 días |
| Fat pointers (ptr + len slices) | 🟡 Media | 2 semanas |
| Cross-compilation targets | 🟢 Baja | 1 semana |
| `no_std` flag para runtime | 🟢 Baja | 1 semana |

---

## 5. Arquitectura propuesta del kernel Byle

```
                    ┌─────────────────────┐
                    │   Byle user programs │
                    │  (compilados con by) │
                    └──────┬──────────────┘
                           │ syscall
                    ┌──────▼──────────────┐
                    │  Kernel Byle (KYOS) │
                    │                     │
                    │  ┌───────────────┐  │
                    │  │ Scheduler     │  │
                    │  │ (round-robin) │  │
                    │  └──────┬────────┘  │
                    │  ┌──────▼────────┐  │
                    │  │ Memory mgmt   │  │
                    │  │ (paging, kmem)│  │
                    │  └──────┬────────┘  │
                    │  ┌──────▼────────┐  │
                    │  │ Device drivers│  │
                    │  │ (UART, disk)  │  │
                    │  └──────┬────────┘  │
                    │  ┌──────▼────────┐  │
                    │  │ Byle runtime  │  │
                    │  │ (bare-metal)  │  │
                    │  └──────┬────────┘  │
                    └─────────┼───────────┘
                              │
                    ┌─────────▼───────────┐
                    │  Boot code (asm)    │
                    │  (multiboot, GDT,   │
                    │   IDT, paging)      │
                    └─────────────────────┘
```

El kernel combina:
- **Boot code en assembly** (Multiboot header, GDT, IDT setup)
- **Capa HAL en Byle** (drivers via `extern fn`, MMIO)
- **Runtime Byle bare-metal** (allocator, strings, lists)
- **Kernel en Byle** (scheduler, syscalls, process management)

---

## 6. Documentos relacionados

- `docs/06-compiler/` → Pipeline del compilador
- `docs/05-runtime/` → Runtime actual (depende de libc)
- `docs/02-getting-started/performance.md` → Tips de rendimiento
- `../kyos/docs/13-freestanding-mode.md` → Documento original de freestanding
- `../kyos/byle-prerequisites/` → Prerrequisitos de Byle para KYOS
