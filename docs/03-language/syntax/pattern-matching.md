# Pattern Matching

**Status:** [x] Basic match with `_:` and literal patterns. [x] `1|2` or-pattern. [x] `..=` range pattern.

## Match statement

```by
match value:
 pattern1:
 body1
 pattern2:
 body2
 _:
 default
```

## Match expression

```by
result = match x:
 1: "one"
 2: "two"
 _: "other"
```

## Patterns

### Literal
```by
match x:
 0: println("zero")
```

### Identifier
```by
match x:
 n: println(n) # binds value to n
```

### Wildcard
```by
match x:
 _: println("anything")
```

### Or-pattern
```by
match x:
 1 | 2: println("one or two")
```

### Guard
```by
match x:
 n if n > 10: println("big")
 n: println(n)
```

### Enum variant
```by
enum Optional:
 Some(i32)
 None

match v:
 Optional.Some(n):
 println(n)
 Optional.None:
 println("none")
```

### IsType
```by
match x:
 is str: println("string")
 is i32: println("integer")
```

## Binding if

```by
if name = optional_value:
 println(name)
```

## Binding while

```by
while line = read_line():
 process(line)
```
