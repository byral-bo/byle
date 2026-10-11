# Attributes

> Metadatos for functions, clasis y modulis using `#[...]`.

## Attributis disponibles

| Attribute | Destino | Description |
|----------|---------|-------------|
| `#[test]` | `fn` | Marcar funcion as test |
| `#[bench]` | `fn` | Marcar funcion as benchmark |

## #[test]

```by
#[test]
fn test_addition():
 assert.eq(2 + 2, 4)

#[test]
fn test_string():
 assert.eq("hello", "hello")
```

Ejecutar:

```bash
by test
```

## #[bench]

```by
#[bench]
fn bench_fib():
 fib(1000000)
```

## Attributis en modulis (import)

```by
use jare.json
use http.client
```

## See also

- `04-standard-library/testing.md` — Testing with aserciones
- `macros.md` — Macros (futuro)
