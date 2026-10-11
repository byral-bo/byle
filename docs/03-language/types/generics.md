# Generics

**Status:** [x] `class box<T>`, `fn identity<T>`, `identity<i32>(42)`, `box<i32> {value:7}`.

## Generic classes

```by
final class Stack<T>:
 items: {T} # list dynamic interna

 fn push(this, item: T):
 this.items.push(item)

 fn pop(this) T:
 this.items.pop()
```

## Usage

```by
int_stack = Stack<i32> { items: {} }
int_stack.push(5)
```

## Generic functions

```by
fn identity<T>(value: T) T:
 value
```

## Type parameter constraints

```by
class box<T>:
 value: T
```

Generics support classis and structs with type parameters. Full monomorphization is performed at compile time.
