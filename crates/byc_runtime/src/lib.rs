#![allow(clippy::not_unsafe_ptr_arg_deref)]

pub mod assert;
pub mod async_;
pub mod big_int;
pub mod bytes;
pub mod channel;
pub mod cli;
pub mod crypto;
pub mod csv;
pub mod date;
pub mod datetime;
pub mod decimal;
pub mod deque;
pub mod dict;
pub mod duration;
pub mod error;
pub mod io;
pub mod linked_list;
pub mod list;
pub mod log;
pub mod memory;
pub mod net;
pub mod panic;
pub mod path_;
pub mod platform;
pub mod queue;
pub mod regex;
pub mod set;
pub mod stack;
pub mod string;
pub mod sync;
pub mod thread;
pub mod url;
pub mod uuid;

pub use async_::{by_await_task, by_spawn_task, by_yield};
pub use big_int::{
    by_big_int_add, by_big_int_free, by_big_int_from_i64, by_big_int_from_str, by_big_int_mul,
    by_big_int_sub, by_big_int_to_str,
};
pub use bytes::{
    by_bytes_free, by_bytes_from_hex, by_bytes_get, by_bytes_new, by_bytes_set, by_bytes_to_base64,
    by_bytes_to_hex,
};
pub use channel::{
    by_channel_close, by_channel_free, by_channel_len, by_channel_new, by_channel_recv,
    by_channel_send,
};
pub use crypto::{by_random_bytes, by_sha256};
pub use date::{
    by_date_add_days, by_date_day, by_date_format, by_date_from_ymd, by_date_month, by_date_parse,
    by_date_today, by_date_weekday, by_date_year, by_time_from_hms, by_time_hour, by_time_minute,
    by_time_now, by_time_parse, by_time_second,
};
pub use datetime::{
    by_datetime_add_days, by_datetime_add_hours, by_datetime_day, by_datetime_diff,
    by_datetime_format, by_datetime_from_ymdhms, by_datetime_hour, by_datetime_minute,
    by_datetime_month, by_datetime_now, by_datetime_parse, by_datetime_second, by_datetime_year,
};
pub use decimal::{by_decimal_from_str, by_decimal_round, by_decimal_to_str, by_decimal_truncate};
pub use deque::{
    by_deque_clear, by_deque_free, by_deque_len, by_deque_new, by_deque_peek_back,
    by_deque_peek_front, by_deque_pop_back, by_deque_pop_front, by_deque_push_back,
    by_deque_push_front,
};
pub use dict::by_dict_keys;
pub use dict::{
    by_dict_contains, by_dict_free, by_dict_get, by_dict_items, by_dict_len, by_dict_new,
    by_dict_remove, by_dict_set, by_dict_values, by_json_to_struct, by_struct_to_json,
};
pub use duration::{
    by_duration_free, by_duration_from_days, by_duration_from_hours, by_duration_from_millis,
    by_duration_from_secs, by_duration_to_str,
};
pub use io::{
    by_close, by_input, by_input_with_prompt, by_now, by_open, by_print, by_println, by_read_str,
    by_sleep, by_write_str,
};
pub use linked_list::{
    by_linked_list_clear, by_linked_list_free, by_linked_list_len, by_linked_list_new,
    by_linked_list_peek_back, by_linked_list_peek_front, by_linked_list_pop_back,
    by_linked_list_pop_front, by_linked_list_push_back, by_linked_list_push_front,
};
pub use list::{
    by_init_args, by_list_chunk, by_list_free, by_list_get, by_list_index_of, by_list_len,
    by_list_new, by_list_pop, by_list_push, by_list_reserve, by_list_set, by_list_sort,
};
pub use memory::{by_alloc, by_free, by_release, by_retain};
pub use net::{
    by_base64_encode, by_ptr_read_i32, by_ptr_read_ptr, by_ptr_write_i32, by_ptr_write_ptr,
    by_sha1, by_tcp_accept, by_tcp_close, by_tcp_listen, by_tcp_read, by_tcp_write, by_ws_accept,
    by_ws_read_frame, by_ws_send_frame,
};
pub use path_::{
    by_path_basename, by_path_dirname, by_path_extension, by_path_free, by_path_join, by_path_new,
    by_path_to_str,
};
pub use queue::{
    by_queue_clear, by_queue_free, by_queue_len, by_queue_new, by_queue_peek, by_queue_pop,
    by_queue_push, by_queue_to_list, by_queue_to_stack,
};
pub use regex::{by_regex_find, by_regex_free, by_regex_is_match, by_regex_new, by_regex_replace};
pub use set::{
    by_set_add, by_set_clear, by_set_contains, by_set_difference, by_set_free, by_set_intersection,
    by_set_is_subset, by_set_len, by_set_new, by_set_remove, by_set_symmetric_difference,
    by_set_to_list, by_set_union,
};
pub use stack::{
    by_stack_clear, by_stack_free, by_stack_len, by_stack_new, by_stack_peek, by_stack_pop,
    by_stack_push, by_stack_to_list, by_stack_to_queue,
};
pub use string::{
    by_char_at, by_char_to_str, by_clone_str, by_clone_substr, by_concat, by_eq_str, by_f32_to_str,
    by_from_cstr, by_getenv, by_i64_to_str, by_is_alnum, by_is_alpha, by_is_digit, by_is_lower,
    by_is_upper, by_is_whitespace, by_ord, by_setenv, by_str_builder_append, by_str_builder_free,
    by_str_builder_new, by_str_builder_to_str, by_str_cmp, by_str_contains, by_str_index_of,
    by_str_replace, by_str_split, by_str_to_i32, by_str_to_i64, by_str_to_list, by_str_to_lower,
    by_str_to_upper, by_str_trim, by_strlen, by_substr, str_builder_append, str_builder_free,
    str_builder_new, str_builder_to_str,
};
pub use thread::{by_join_thread, by_spawn_thread};
pub use url::{by_url_host, by_url_path, by_url_port, by_url_query, by_url_scheme};
pub use uuid::{by_uuid_parse, by_uuid_v4};

pub use sync::{
    by_atomic_bool_free, by_atomic_bool_load, by_atomic_bool_new, by_atomic_bool_store,
    by_atomic_i64_add, by_atomic_i64_free, by_atomic_i64_load, by_atomic_i64_new,
    by_atomic_i64_store, by_mutex_free, by_mutex_lock, by_mutex_new, by_mutex_store,
};

pub use log::{
    by_log_debug, by_log_error, by_log_info, by_log_set_console, by_log_set_level,
    by_log_set_output, by_log_warn,
};

pub use cli::{
    by_cli_arg, by_cli_argc, by_cli_define, by_cli_get, by_cli_get_bool, by_cli_get_int,
    by_cli_has, by_cli_help, by_cli_parse,
};

pub use csv::{
    by_csv_col_count, by_csv_free, by_csv_get, by_csv_get_col, by_csv_parse, by_csv_row_count,
    by_csv_to_str,
};

/// Power: compute base ** exp for i64 values. Returns i64 (truncated).
#[unsafe(no_mangle)]
pub extern "C" fn by_pow(base: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp < 0 {
        return 0;
    } // floor for negative exponents
    let mut result: i64 = 1;
    for _ in 0..exp {
        result = result.wrapping_mul(base);
    }
    result
}

/// `x +% p` = x + (x * p / 100)
#[unsafe(no_mangle)]
pub extern "C" fn by_add_pct(x: i64, p: i64) -> i64 {
    x + (x * p / 100)
}

/// `x -% p` = x - (x * p / 100)
#[unsafe(no_mangle)]
pub extern "C" fn by_sub_pct(x: i64, p: i64) -> i64 {
    x - (x * p / 100)
}

/// `x *% p` = x * p / 100 (percentage of)
#[unsafe(no_mangle)]
pub extern "C" fn by_mul_pct(x: i64, p: i64) -> i64 {
    x * p / 100
}
