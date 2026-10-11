# json — JSON serialization

> Serialize and deserialize classes to/from JSON. Class-based, no manual parsing.
> Import: `use std.json`

## Serialize a class

```by
use std.json

class User:
    name: str
    age: i32
    active: bool

user: User = User { name: "Byle", age: 30, active: true }
json_str: str = json.to_str(user)
println(json_str)
# {"name":"Byle","age":30,"active":true}
```

## Deserialize to a class

```by
class User:
    name: str
    age: i32
    active: bool

json_str: str = """{"name": "Byle", "age": 30, "active": true}"""
user: User = json.from_str<User>(json_str)!
println(user.name)
println(user.age.to_str())
```

## Pretty print

```by
user: User = User { name: "Byle", age: 30 }
formatted: str = json.pretty(user)
println(formatted)
# {
#   "name": "Byle",
#   "age": 30
# }
```

## Lists of classes

```by
class User:
    name: str
    age: i32

users: [User] = [
    User { name: "Ana", age: 25 },
    User { name: "Bob", age: 35 }
]

json_str: str = json.to_str(users)
# [{"name":"Ana","age":25},{"name":"Bob","age":35}]

parsed: [User] = json.from_str<[User]>(json_str)!
for user in parsed:
    println(user.name)
```

## Nested classes

```by
class Address:
    city: str
    country: str

class User:
    name: str
    address: Address

user: User = User {
    name: "Byle",
    address: Address { city: "NYC", country: "USA" }
}

json_str: str = json.to_str(user)
# {"name":"Byle","address":{"city":"NYC","country":"USA"}}

parsed: User = json.from_str<User>(json_str)!
println(parsed.address.city)
```

## Dictionaries in JSON

```by
metadata: {str: str} = {"version": "1.0", "env": "production"}
json_str: str = json.to_str(metadata)
# {"version":"1.0","env":"production"}
```

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `to_str(val)` | `fn(val: T) str` | Serialize class/list/dict to JSON |
| `from_str<T>(s)` | `fn(s: &str) T` | Deserialize JSON string to type T |
| `pretty(val)` | `fn(val: T) str` | Pretty-print with indentation |

## Type mapping

| Byle | JSON |
|------|------|
| `str` | string |
| `i32`, `i64` | number |
| `f64` | number |
| `bool` | true / false |
| `none` | null |
| `[T]` | list |
| `{K: V}` | object |
| `class` | object |

## Example

```by
use std.json

class Config:
    host: str
    port: i32
    debug: bool

content: str = std.fs.read("config.json")!
config: Config = json.from_str<Config>(content)!
println("connecting to " + config.host + ":" + config.port.to_str())
```
