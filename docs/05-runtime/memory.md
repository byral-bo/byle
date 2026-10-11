# Memory Management

> Management de memory en runtime de Byle: allocation, deallocation, retain/release.
> Crate: `byc_runtime/src/memory.rs` (67 lines).

## Responsabilidad

El runtime de Byle proporciona management manual de memory for strings, lists, dictionarys
y otros typis heap-allocados. No there is garbage collector — memory se libera explicitamente
using borrow analysis del compiler, que inserta llamadas a `by_free` automaticamente.

## Functions

### by_alloc

```rust
#[unsafe(no_mangle)]
 extern "C" fn by_alloc(size: i64) -> *mut u8
```

Asigna un bloque de memory del heap del size especificado.

- Retorna un pointer al bloque asignado, o null si fails
- La memory incluye un header with refcount y size
- Equivalente a `malloc()` de C

```by
extern fn by_alloc(size: i64) ptr
buf = by_alloc(1024) # asigna 1024 bytes
```

### by_free

```rust
#[unsafe(no_mangle)]
 extern "C" fn by_free(ptr: *mut u8)
```

Libera un bloque de memory previamente asignado with `by_alloc`.

- No does nada si `ptr` is null
- No verifica doble-free (undefined behavior)
- Equivalente a `free()` de C

```by
extern fn by_free(ptr)
by_free(buf) # libera memory
```

### by_retain / by_release

```rust
#[unsafe(no_mangle)]
 extern "C" fn by_retain(ptr: *mut u8)
#[unsafe(no_mangle)]
 extern "C" fn by_release(ptr: *mut u8)
```

Sistema de reference counting for memory compartida.

- `by_retain`: incrementa contador de referencias atomico
- `by_release`: decrementa y libera si llega a 0
- Usa `AtomicI64` for contador

### Header

Cada bloque de memory has un header con:

```rust
struct Header {
 strong: AtomicI64, // reference count
 size: i64, // size del bloque
}
```

## Integration with borrow analysis

El compiler (`borrow_analysis.rs`) determina automaticamente cuando insertar `by_free`:

```rust
// Generado by compiler al final del scope
by_free(s_ptr) # str sale de scope → free
by_list_free(lst) # list sale de scope → free (llama by_free internamente)
by_dict_free(d) # dictionary sale de scope → free
```

Esto elimina necesidad de `free()` manual o garbage collector.

## Estrategia de allocation

- `by_alloc` usa `Layout::from_size_align` de Rust (que usa `malloc`/`free` del sistema)
- Strings, lists y dictionarys usan `by_alloc` internamente
- El runtime NO has su propio pool/arena — delega en asignador del sistema

## See also

- `allocator.md` — Details del asignador de memory
- `06-compiler/borrow-analysis.md` — How compiler inserta `by_free`
