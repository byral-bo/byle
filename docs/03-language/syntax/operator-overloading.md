# Operator Overloading

**Status:** Implemented
**Related:** [expressions.md](expressions.md), [operators.md](../lexical/operators.md)

---

## 1. Qué operadores se pueden sobrecargar

Byle permite sobrecargar operadores implementando métodos con nombres
específicos en una clase.

### 1.1 Operadores aritméticos

| Operador | Método | Ejemplo |
|----------|--------|---------|
| `+` | `op_add(other: T) Self` | `a + b` |
| `-` | `op_sub(other: T) Self` | `a - b` |
| `*` | `op_mul(other: T) Self` | `a * b` |
| `/` | `op_div(other: T) Self` | `a / b` |
| `%` | `op_mod(other: T) Self` | `a % b` |
| `**` | `op_pow(other: T) Self` | `a ** b` |

### 1.2 Operadores de comparación

| Operador | Método | Ejemplo |
|----------|--------|---------|
| `==` | `op_eq(other: T) bool` | `a == b` |
| `!=` | `op_ne(other: T) bool` | `a != b` |
| `<` | `op_lt(other: T) bool` | `a < b` |
| `>` | `op_gt(other: T) bool` | `a > b` |
| `<=` | `op_le(other: T) bool` | `a <= b` |
| `>=` | `op_ge(other: T) bool` | `a >= b` |

### 1.3 Operadores unarios

| Operador | Método | Ejemplo |
|----------|--------|---------|
| `-` | `op_neg() Self` | `-a` |
| `not` | `op_not() bool` | `not a` |

### 1.4 Operadores de bit

| Operador | Método | Ejemplo |
|----------|--------|---------|
| `and` (bitwise) | `op_bitand(other: T) Self` | `a and b` |
| `or` (bitwise) | `op_bitor(other: T) Self` | `a or b` |
| `xor` | `op_xor(other: T) Self` | `a xor b` |
| `<<` | `op_shl(other: T) Self` | `a << b` |
| `>>` | `op_shr(other: T) Self` | `a >> b` |

### 1.5 Operadores de indexación

| Operador | Método | Ejemplo |
|----------|--------|---------|
| `[]` | `op_index(key: K) V` | `dict[key]` |
| `[]=` | `op_index_set(key: K, value: V)` | `dict[key] = val` |

---

## 2. Ejemplo: Vector 2D

```byle
final class Vec2:
    x: f32
    y: f32

    # Aritmética
    fn op_add(this, other: Vec2) Vec2:
        Vec2(x: this.x + other.x, y: this.y + other.y)

    fn op_sub(this, other: Vec2) Vec2:
        Vec2(x: this.x - other.x, y: this.y - other.y)

    fn op_mul(this, scalar: f32) Vec2:
        Vec2(x: this.x * scalar, y: this.y * scalar)

    # Comparación
    fn op_eq(this, other: Vec2) bool:
        this.x == other.x and this.y == other.y

    # Unario
    fn op_neg(this) Vec2:
        Vec2(x: -this.x, y: -this.y)
```

```byle
a = Vec2(x: 1.0, y: 2.0)
b = Vec2(x: 3.0, y: 4.0)

c = a + b        # Vec2(4.0, 6.0)
d = a - b        # Vec2(-2.0, -2.0)
e = a * 2.0      # Vec2(2.0, 4.0)
f = -a           # Vec2(-1.0, -2.0)
g = a == b       # false
```

---

## 3. Ejemplo: Número complejo

```byle
final class Complex:
    real: f64
    imag: f64

    fn op_add(this, other: Complex) Complex:
        Complex(this.real + other.real, this.imag + other.imag)

    fn op_mul(this, other: Complex) Complex:
        Complex(
            this.real * other.real - this.imag * other.imag,
            this.real * other.imag + this.imag * other.real,
        )

    fn op_eq(this, other: Complex) bool:
        this.real == other.real and this.imag == other.imag

    fn to_str(this) str:
        this.real.to_str() + " + " + this.imag.to_str() + "i"
```

---

## 4. Ejemplo: Matriz

```byle
final class Mat2:
    data: [f32, 4]  # 2x2 matrix en row-major

    fn op_index(this, row: i32, col: i32) f32:
        this.data[row * 2 + col]

    fn op_index_set(this, row: i32, col: i32, value: f32):
        this.data[row * 2 + col] = value
```

```byle
m = Mat2(data: [1.0, 2.0, 3.0, 4.0])
val = m[0, 1]       # 2.0
m[1, 0] = 5.0       # set
```

---

## 5. Reglas

| Regla | Descripción |
|-------|-------------|
| **Tipado estricto** | Los parámetros y retorno deben coincidir con la firma esperada |
| **No mezclar tipos** | `a + b` requiere que ambos sean del mismo tipo (o `op_add` acepte `other: T`) |
| **Operadores built-in** | Tipos primitivos (`i32`, `f64`, `str`) tienen operadores predefinidos no sobrecargables |
| **Comparación** | `op_eq` debe retornar `bool` |
| **Indexación** | `op_index` y `op_index_set` pueden tener cualquier tipo de key/value |

---

## 6. Referencias

- [expressions.md](expressions.md) — Operadores y precedencia
- [operators.md](../lexical/operators.md) — Operadores built-in
- [structs.md](../types/structs.md) — Definición de clases
