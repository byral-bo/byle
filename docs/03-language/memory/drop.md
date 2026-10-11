# Drop / Free

> Los valueis Move se liberan automaticamente al salir de scope.
> El compiler inserta llamadas a `by_free` where corresponde.

## Automatic

No necesitas llamar `free()` manualmente. El borrow analysis inserta `by_free`
al final del scope:

```by
fn main() i32:
 s: str = "hola"
 println(s)
 # ← aqui compiler inserta by_free(s)
 0
```

## Equivalente manual (for FFI)

Cuando trabajas with pointers raw (`ptr`), management is manual:

```by
extern fn by_alloc(size: i64) ptr
extern fn by_free(ptr)

buf: ptr = by_alloc(1024)
# usar buf...
by_free(buf)
```

## Scope y free

```by
fn test() i32:
 if true:
 s: str = "temporal"
 # ← by_free(s) aqui (sale de scope del if)
 println("ok")
 # without error: s ya se libero
 0
```

## Orden de deallocation

Las variablis se liberan en orden inverso a su creation (LIFO):

```by
a: str = "primero"
b: str = "segundo"
# ← by_free(b)
# ← by_free(a)
```

## See also

- `allocator.md` — How funciona allocador
- `move.md` — When se mueve ownership
