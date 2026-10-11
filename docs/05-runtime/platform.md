# Platform Abstraction

> Capa de abstraccion de plataforma: interaccion with sistema operativo.
> Crate: `byc_platform` (en desarrollo).

## Responsabilidad

La capa Platform abstrae diferencias between sistemas operativos, proporcionando
una API unificada for runtime de Byle. Es unica capa que conoce SO subyacente.

## Status current

| Module | Status | Crate | Notis |
|--------|--------|-------|-------|
| File I/O | ✅ | `byc_runtime/src/io.rs` | open/read/write/close |
| Time | ✅ | `byc_runtime/src/datetime.rs` | now/parse/format |
| Networking | ✅ | `byc_runtime/src/net.rs` | TCP listen/accept/read/write |
| Threads | ✅ | `byc_runtime/src/thread.rs` | spawn/join |
| Environment | ✅ | `byc_runtime/src/string.rs` | getenv/setenv |
| Filesystem (dirs) | ❌ | — | Pending |
| Process | ❌ | — | Pending |
| Signals | ❌ | — | Pending |
| Sockets (UDP) | ❌ | — | Pending |
| USB/Serial | ❌ | — | Pending |

## I/O Module

El module I/O current (`io.rs`) proporciona:

```rust
// File operations
by_open(path, mode) -> i32 fd // Open file
by_read_str(fd, len) -> ptr // Read as string
by_write_str(fd, buf) -> i32 // Write string
by_close(fd) -> i32 // Close fd
```

## Networking Module

```rust
// TCP operations
by_tcp_listen(port) -> i32 fd
by_tcp_accept(fd) -> i32 client_fd
by_tcp_read(fd, len) -> ptr
by_tcp_write(fd, buf, len) -> i32
by_tcp_close(fd) -> i32

// WebSocket
by_ws_accept(request) -> ptr
by_ws_read_frame(fd) -> ptr
by_ws_send_frame(fd, opcode, data, len) -> i32

// Crypto helpers
by_sha1(data, len, output) -> i32
by_base64_encode(data, len) -> ptr

// Pointer operations (for FFI)
by_ptr_read_i32(ptr) -> i32
by_ptr_read_ptr(ptr) -> ptr
by_ptr_write_i32(ptr, i32)
```

## Time Module

```rust
by_now() -> i64 // Current timestamp (ms)
by_sleep(ms) // Sleep milliseconds
```

## Platform Adapters

Cada SO has su propia implementation de plataforma:

| SO | Status |
|----|--------|
| macOS (Apple Silicon) | ✅ Production |
| Linux (ARM64) | ✅ CI tested |
| Linux (x86_64) | ✅ CI tested |
| WASM | 🔶 Experimental |
| Windows | 📅 Planned |
| KYOS | 📅 Aspirational |

## See also

- `06-compiler/linker.md` — How se linkea with libraries de plataforma
- `04-standard-library/io.md` — API de I/O for usuario
- `04-standard-library/fs.md` — API de filesystem
