# Variables

**Status:** [x] Documentación completa. [~] Parcialmente implementado.

## Declaration

```by
x: i32 = 42         # type explícito + value
y = 10              # type inferido (i32)
name: str = "Ana"
```

No hay `let`, `var` ni `const`.

## Inmutabilidad por defecto

```by
x: i32 = 10
x = 20   # ERROR
```

### Mutable con `^`

```by
x: ^i32 = 10
x = x + 1          # ✅

items: ^[str] = [] # lista mutable
items.push("a")
```

### Borrow con `&`

```by
x: &str = &"hello"
items: &[i32] = &[1, 2, 3]
```

### Mutable borrow `^&`

```by
x: ^&str
items: ^&[i32]
```

## Option `?` y Error `!`

Ortogonales: funcionan en CUALQUIER tipo.

```by
x: i32?          # Option<i32>
x: [str]?        # lista opcional
x: ^[i32]!       # lista mutable con error
x: ^&{str: i32}? # dict mutable borrow opcional
x: queue<i32>!   # queue con error
```

## Copy vs Move

### Copy (i32, f64, bool, char, ptr, [T, N])

```by
x = 42
y = x   # COPY: ambos vivos
```

### Move (str, [T], {K:V}, set<T>, queue<T>, clases)

```by
s = "hola"
t = s   # MOVE: s inválido
```

## Globals

```by
print("hello")
name = input("name? ")
```

## Casting

```by
x = 42 as f64
```

## Scope

```by
x = 1
if true:
    y = 2
    x = x + y
```

## Destructuring

```by
punto: (i32, str) = (10, "hello")
(x, y) = punto
```
