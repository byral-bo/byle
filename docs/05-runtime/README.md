# 05-runtime

> Documentation del runtime de Byle: libreria static que se linkea a todo binary compilado.

## Files

| Documento | Description | Status |
|-----------|-------------|--------|
| `memory.md` | Assignment y deallocation de memory (`by_alloc`/`by_free`) | ✅ |
| `allocator.md` | Estrategia de allocation y limitacionis | ✅ |
| `scheduler.md` | Thread pool y async/await | ✅ |
| `panic.md` | Manejo de errors fatalis | ✅ |
| `startup.md` | Secuencia de inicio del runtime | ✅ |
| `platform.md` | Abstraction de plataforma (OS) | ✅ |

## Modulis del runtime

| Module (Rust) | Lines | Functions exportadas |
|---------------|--------|---------------------|
| `memory.rs` | 67 | `by_alloc`, `by_free`, `by_retain`, `by_release` |
| `io.rs` | 202 | `by_print`, `by_println`, `by_input`, `by_open`, `by_read_str`, `by_write_str`, `by_close`, `by_sleep`, `by_now` |
| `string.rs` | 584 | `by_strlen`, `by_concat`, `by_i64_to_str`, `by_str_builder_*`, `by_str_*`, `by_getenv`/`by_setenv` |
| `list.rs` | 595 | `by_list_new`, `by_list_push`, `by_list_pop`, `by_list_get`, `by_list_set`, `by_list_len`, `by_list_map`, `by_list_filter`, `by_iter_*` |
| `dict.rs` | 400 | `by_dict_new`, `by_dict_get`, `by_dict_set`, `by_dict_len`, `by_struct_to_json`, `by_json_to_struct` |
| `net.rs` | 285 | `by_tcp_listen`, `by_tcp_read`, `by_tcp_write`, `by_ws_*`, `by_ptr_*`, `by_sha1`, `by_base64_encode` |
| `async_.rs` | 152 | `by_spawn_task`, `by_await_task`, `by_yield` |
| `channel.rs` | 191 | `by_channel_new`, `by_channel_send`, `by_channel_recv`, `by_channel_close`, `by_channel_len`, `by_channel_free` |
| `thread.rs` | 19 | `by_spawn_thread`, `by_join_thread` |
| `datetime.rs` | 192 | `by_datetime_now/parse/format/from_ymdhms` |
| `date.rs` | 143 | `by_date_today/from_ymd/parse` |
| `bytes.rs` | 95 | `by_bytes_new/get/set/to_hex/from_hex/to_base64` |
| `decimal.rs` | 58 | `by_decimal_from_str/to_str/round/truncate` |
| `uuid.rs` | 41 | `by_uuid_v4/parse` |
| `url.rs` | 47 | `by_url_scheme/host/port/path/query` |
| `regex.rs` | 91 | `by_regex_new/is_match/find/replace` |
| `panic.rs` | 6 | `by_panic` |
| `task.rs` | 41 | Interno: `PollState`, `BoxedFuture` |

**Total: ~3350 lines de Rust. 88 functions `extern "C"` exportadas.**
