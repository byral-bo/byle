# Enums

**Status:** [x] Enum variants, match with `Enum.Variant`, `_:` catch-all.

Enums are tagged unions (discriminated unions).

## Declaration

```by
enum Color:
 Red
 Green
 Blue
```

## With payload

```by
enum Optional:
 Some(i32)
 None
```

## Usage

```by
c = Color.Red

match c:
 Color.Red:
 println("red")
 Color.Green:
 println("green")
 Color.Blue:
 println("blue")
```

## Variant with payload

```by
v = Optional.Some(42)

match v:
 Optional.Some(n):
 println(n)
 Optional.None:
 println("none")
```
