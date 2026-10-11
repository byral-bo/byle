# Statements

**Status:** [x] if/elif/else, while, for-in range, match, return, break, continue, defer, guard.

## Expression statement

```by
x + 1
println("hello")
```

## Block

```by
:
 statement1
 statement2
```

## If / elif / else

```by
if x > 0:
 println("positive")
elif x < 0:
 println("negative")
else:
 println("zero")
```

## While

```by
i: ^i32 = 0
while i < 10:
 println(i)
 i = i + 1
```

## For

```by
# Range — itera por valor (Copy types se copian)
for i in 0..10:
 println(i)

# Lista — itera por MOVE (consume la lista, cada valor se mueve)
for libro in libros:
 println(libro)

# Lista — itera por BORROW (no consume)
for libro in &libros:
 println(libro)

# Lista — itera por MUT BORROW (modificar elementos)
for libro in ^&libros:
 if libro.contains("x"):
     libro.push("editado")

# Array — itera por valor (copias en stack)
for val in arr:
 println(val)

# Array — itera por borrow (sin copiar el array)
for val in &arr:
 println(val)

# Array — itera por mut borrow (modificar elementos)
for val in ^&arr:
 val = val * 2

# Inclusive range
for i in 0..=5:
 println(i)

# For-else: runs if loop completes without break
for i in 0..10:
 println(i)
else:
 println("done")
```

## Match

```by
match x:
 1:
 println("one")
 2 | 3:
 println("two or three")
 n if n > 10:
 println("big")
 _:
 println("other")
```

## Return

```by
fn add(a: i32, b: i32) i32:
 return a + b # explicit
 a + b # implicit (last expression)
```

## Break / continue

```by
while true:
 if done:
 break
 i = i + 1
 if skip:
 continue
```

## Defer

```by
file = open("data.txt")
defer close(file) # runs when scope exits
```

## Guard

```by
guard value = optional else:
 return
```

## Unsafe

```by
unsafe:
 ptr = as_ptr(variable)
 ptr[0] = 0xFF
```
