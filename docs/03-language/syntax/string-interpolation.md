# String Interpolation

**Status:** Implemented
**Related:** [expressions.md](expressions.md), [literals.md](../lexical/literals.md)

---

## 1. Sintaxis

Byle soporta interpolación de strings con la sintaxis `{expresión}` dentro
de strings delimitados por comillas dobles:

```byle
name = "Juan"
age = 30
greeting = "Hola, {name}, tienes {age} años"
# → "Hola, Juan, tienes 30 años"
```

### 1.1 Expresiones arbitrarias

Se puede interpolar cualquier expresión Byle:

```byle
"El total es {price * 1.21}"
"Usuario: {user.name} (ID: {user.id})"
"Resultado: {if ok: "OK" else: "ERROR"}"
"Promedio: {numbers.sum() / numbers.len()}"
```

### 1.2 Anidamiento

```byle
"Prefijo {fn(nested())} sufijo"
```

---

## 2. Escaping

| Secuencia | Resultado |
|-----------|-----------|
| `\{` | `{` literal |
| `\}` | `}` literal |
| `\\` | `\` literal |

```byle
"Llaves: \{esto no se interpola\}"
# → "Llaves: {esto no se interpola}"
```

---

## 3. Conversión a str

Las expresiones interpoladas se convierten automáticamente a `str` llamando
al método `.to_str()` del tipo:

```byle
count = 42
"Valor: {count}"     # llama count.to_str() automáticamente
```

Si el tipo no tiene `.to_str()`, es **error de compilación**.

---

## 4. Ejemplos

```byle
# Variables simples
name = "Ana"
"Hola, {name}"

# Llamadas a métodos
"El usuario es {user.get_full_name()}"

# Aritmética
"Total: {subtotal + tax}"

# Condicional inline
"Estado: {if active: "Activo" else: "Inactivo"}"

# Con literales
"Path: /users/{id}/edit"

# Múltiples líneas
message = """
    Usuario: {user.name}
    Email: {user.email}
    Rol: {user.role}
"""
```

---

## 5. Rendimiento

La interpolación se compila a concatenación de strings en tiempo de
compilación:

```byle
"Hola, {name}, tienes {age} años"
```

→ Se compila a:

```byle
"Hola, " + name.to_str() + ", tienes " + age.to_str() + " años"
```

---

## 6. Referencias

- [expressions.md](expressions.md) — Expresiones
- [literals.md](../lexical/literals.md) — Literales de string
- [operators.md](../lexical/operators.md) — Operador `+` para strings
