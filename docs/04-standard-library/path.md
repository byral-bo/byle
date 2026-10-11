# path — Path manipulation

> Platform-independent path handling.
> Import: `use std.path`

## Creating paths

```by
use std.path

p: path = path("/home/user/file.txt")
p = path.join("data", "images", "photo.jpg")
```

## Path components

```by
p: path = path("/home/user/file.txt")

dir: str = p.dirname()       # "/home/user"
base: str = p.basename()     # "file.txt"
ext: str = p.extension()     # ".txt"
stem: str = p.stem()         # "file"
```

## Checking existence

```by
if p.exists():
    if p.is_file():
        println("file")
    elif p.is_dir():
        println("directory")
```

## Joining

```by
p: path = path("/data")
p = p.join("images").join("icons").join("photo.jpg")
```

## Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `path(s)` | `fn(s: &str) path` | Create path from string |
| `path.join(...parts)` | `fn(...parts: &str) path` | Join path components |
| `p.dirname()` | `fn() str` | Parent directory |
| `p.basename()` | `fn() str` | File name with extension |
| `p.extension()` | `fn() str` | Extension including dot |
| `p.stem()` | `fn() str` | File name without extension |
| `p.exists()` | `fn() bool` | True if path exists |
| `p.is_file()` | `fn() bool` | True if path is a file |
| `p.is_dir()` | `fn() bool` | True if path is a directory |
| `p.join(other)` | `fn(other: &str) path` | Append path component |
| `p.to_str()` | `fn() str` | Full path as string |

## Example

```by
use std.path

p: path = path("/data")
p = p.join("images").join("photo.jpg")
if p.exists():
    println("file: " + p.to_str())
    println("extension: " + p.extension())
```
