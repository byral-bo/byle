# Diagnostics

> Sistema de errors y diagnosticos del compiler Byle.
> Reporta errors with code, mensaje y ubicacion precisa.

## Formato de error

```
BYLE-E0001: Type mismatch

 expected 'i32', found 'str'

 --> file.by:10:5
 |
10 | x = "hello" + 42
 | ^^^^^^^^^^^^^ expected i32 here
 |
```

## Codis de error

| Code | Significado |
|--------|-------------|
| `BYLE-E0001` | Type mismatch |
| `BYLE-E0002` | Syntax error |
| `BYLE-E0003` | Undefined type |
| `BYLE-E0004` | Cannot modify immutable |
| `BYLE-E0005` | Undefined function |
| `BYLE-E0006` | Wrong number of arguments |
| `BYLE-E0007` | Cannot modify constant |
| `BYLE-E0008` | Invalid assignment target |
| `BYLE-E0009` | Undefined symbol |
| `BYLE-E0010` | Wrong number of type arguments |
| `BYLE-E0011` | Ambiguous type |
| `BYLE-E0012` | Return type mismatch |
| `BYLE-E0013` | Move analysis error |

## Warnings

| Warning | Condition |
|---------|-----------|
| Variable no usada | Declarada pero nunca leida |
| Import no usado | Module importado no utilizado |
| Code muerto | Code after de `return`/`break` |
| Conversion implicita | Sin `as` cast explicito |

## Move analysis errors

El borrow checker produce errors especificos:

```
BYLE-E0013: use-after-move: cannot move `s` (local #2) — value has been moved
BYLE-E0013: cannot mutably borrow `s` — it is already borrowed immutably
```

## See also

- `06-compiler/diagnostics.md` — Implementation del sistema de diagnosticos
- `06-compiler/borrow-analysis.md` — Errors del borrow checker
