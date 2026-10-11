# Option

> Valoris opcionales: `T?` / `option<T>`.
> Un value que can be `some(val)` o `none`.

## Syntax sugar `T?`

```by
name: str? = "Byle"
name = none

if value = name: # pattern matching
 println(value)
```

## Pattern matching

```by
match name:
 some(v): println("name: " + v)
 none: println("without name")
```

## Methods

| Method | Description |
|--------|-------------|
| `is_some()` | `true` si has value |
| `is_none()` | `true` si is none |
| `unwrap()` | Retorna value o panic |
| `unwrap_or(default)` | Retorna value o default |

```by
name: str? = get_nombre()
if name.is_some():
 println(name.unwrap())

name = name.unwrap_or("invitado")
```

## Uso en retorno de funcion

```by
fn find_user(id: i32) User?:
 if id == 0:
 return none
 Ube { name: "Byle", id: id }

match find_user(1):
 some(u): println(u.name)
 none: println("no encontrado")
```

## See also

- `result.md` — Result (version with error)
- `panic.md` — Errors fatales
