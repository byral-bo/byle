# cli — Command-line argument parsing

> Parse command-line flags and positional arguments.
> Import: `use std.cli`

## Quick start

```by
use std.cli

args: {str: str} = cli.parse()
name: str = args["name"] ?? "world"
verbose: bool = cli.has("verbose")
println("hello " + name)
```

## Running

```bash
by run app.by --name=Byle --verbose
# hello Byle
```

## Positional arguments

```by
use std.cli

cli.parse()  # must be called first
first: str = cli.arg(0)
second: str = cli.arg(1)
println("first: " + first + ", second: " + second)
```

```bash
by run app.by input.txt output.txt
# first: input.txt, second: output.txt
```

## Default values

```by
args: {str: str} = cli.parse()
port: str = args["port"] ?? "8080"
host: str = args["host"] ?? "localhost"
mode: str = args["mode"] ?? "release"
```

```bash
by run app.by --port=3000
# port=3000, host=localhost, mode=release
```

## Flag definitions with type hints

```by
cli.define("port", "p", "server port", "8080")
cli.define("host", "h", "server host", "localhost")
cli.define("verbose", "v", "enable verbose output", "false")

args: {str: str} = cli.parse()

port: i32 = cli.get_int("port")
verbose: bool = cli.get_bool("verbose")
```

```bash
by run app.by -p 9000 -v
```

## Help text

If you define flags, `--help` is auto-generated:

```by
cli.define("port", "p", "server port", "8080")
cli.define("host", "h", "server host", "localhost")
cli.parse()
```

```bash
by run app.by --help
# Usage: app.by [options]
#   --port, -p   server port       (default: 8080)
#   --host, -h   server host       (default: localhost)
```

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `parse()` | `fn() {str: str}` | Parse CLI args, returns flag dict |
| `arg(n)` | `fn(n: i32) str` | Get positional argument at index |
| `has(name)` | `fn(name: &str) bool` | Check if flag was provided |
| `get_int(name)` | `fn(name: &str) i32` | Get flag as integer (with default) |
| `get_bool(name)` | `fn(name: &str) bool` | Get flag as boolean |
| `define(name, short, desc, default)` | `fn(name: &str, short: &str, desc: &str, default: &str)` | Define a flag with metadata |

## Example

```by
use std.cli

cli.define("port", "p", "server port", "8080")
cli.define("db", "d", "database URL", "localhost")
cli.define("verbose", "v", "verbose output", "false")

args: {str: str} = cli.parse()

port: i32 = cli.get_int("port")
db: str = args["db"]
verbose: bool = cli.get_bool("verbose")

println("starting on port " + port.to_str())
if verbose:
    println("database: " + db)
```
