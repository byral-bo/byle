use byc_core::ast::{
    ClassDecl, ContractDecl, EnumDecl, FunctionDecl, StructDecl, TypeAlias, Visibility,
};
use byc_core::types::Type;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum SymKind {
    Variable {
        type_: Option<Type>,
        is_mutable: bool,
        is_auto: bool,
    },
    Constant(Type),
    Function(FunctionDecl),
    Class(ClassDecl),
    Struct(StructDecl),
    Enum(EnumDecl),
    Contract(ContractDecl),
    TypeAlias(TypeAlias),
    Module(Vec<(String, SymKind)>),
    TypeParam,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub name: String,
    pub kind: SymKind,
}

impl Symbol {
    pub fn new(name: String, kind: SymKind) -> Self {
        Self { name, kind }
    }

    pub fn new_var(name: String, type_: Option<Type>, is_mutable: bool) -> Self {
        Self {
            name,
            kind: SymKind::Variable {
                type_,
                is_mutable,
                is_auto: false,
            },
        }
    }

    pub fn new_auto(name: String, type_: Option<Type>) -> Self {
        Self {
            name,
            kind: SymKind::Variable {
                type_,
                is_mutable: false,
                is_auto: true,
            },
        }
    }
}

#[derive(Default)]
pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
    pub type_defs: HashMap<String, Type>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut st = Self::default();
        st.scopes.push(HashMap::new());
        st.register_builtins();
        st
    }

    fn register_builtins(&mut self) {
        let builtin_types = [
            ("i8", Type::I8),
            ("i16", Type::I16),
            ("i32", Type::I32),
            ("i64", Type::I64),
            ("u8", Type::U8),
            ("u16", Type::U16),
            ("u32", Type::U32),
            ("u64", Type::U64),
            ("f32", Type::F32),
            ("f64", Type::F64),
            ("bool", Type::Bool),
            ("char", Type::Char),
            ("str", Type::Str),
            ("void", Type::Void),
            ("ptr", Type::Ptr),
            (
                "Result",
                Type::Object(vec![
                    ("disc".to_string(), Type::I32),
                    ("payload".to_string(), Type::I64),
                ]),
            ),
        ];
        for (name, ty) in &builtin_types {
            self.type_defs.insert(name.to_string(), ty.clone());
        }
        // Runtime built-in functions (resolved by type_checker, registered here
        // to avoid "undefined symbol" errors from the scope resolver).
        let runtime_fns = [
            "print",
            "println",
            "print_err",
            "len",
            "input",
            "range",
            "json_parse",
            "json_stringify",
            "json_stringify_str",
            "serialize",
            "deserialize",
            "type",
            "by_struct_to_json",
            "by_json_to_struct",
            "by_ptr_read_i32",
            "by_ptr_read_ptr",
            "open",
            "read_str",
            "write_str",
            "close",
            "sleep",
            "now",
            "assert",
            "assert_eq",
            "assert_ne",
            "assert_str",
            "contains",
            "to_upper",
            "to_lower",
            "trim",
            "replace",
            "substr",
            "char_at",
            "is_digit",
            "is_alpha",
            "is_alnum",
            "is_whitespace",
            "is_upper",
            "is_lower",
            "ord",
            "error",
            "ceil",
            "floor",
            "round",
            "push",
            "list_new",
            "list_len",
            "list_get",
            "list_set",
            "list_pop",
            "list_push",
            "reserve",
            "by_spawn_thread",
            "by_join_thread",
            "by_parallel_for",
            "ok",
            "some",
            "by_log_debug",
            "by_log_info",
            "by_log_warn",
            "by_log_error",
            "by_log_set_level",
            "by_log_set_output",
            "by_log_set_console",
            "by_channel_new",
            "by_channel_send",
            "by_channel_recv",
            "by_channel_close",
            "by_channel_len",
            "by_channel_free",
            "by_bytes_new",
            "by_bytes_free",
            "by_bytes_get",
            "by_bytes_set",
            "by_bytes_to_hex",
            "by_bytes_from_hex",
            "by_bytes_to_base64",
            "by_bytes_from_base64",
            "by_bytes_to_be_i32",
            "by_bytes_to_le_i32",
            "by_bytes_to_be_i64",
            "by_bytes_to_le_i64",
            "by_bytes_from_be_i32",
            "by_bytes_from_le_i32",
            "by_bytes_from_be_i64",
            "by_bytes_from_le_i64",
            "by_bytes_from_be_i32_at",
            "by_bytes_from_be_i64_at",
            "by_buffer_new",
            "by_buffer_free",
            "by_buffer_len",
            "by_buffer_write_byte",
            "by_buffer_write_be_i32",
            "by_buffer_write_le_i32",
            "by_buffer_write_be_i64",
            "by_buffer_write_le_i64",
            "by_buffer_write_str",
            "by_buffer_to_bytes",
            "by_buffer_clear",
            "by_str_builder_new",
            "by_str_builder_append",
            "by_str_builder_to_str",
            "by_str_builder_free",
            "str_builder_new",
            "str_builder_append",
            "str_builder_to_str",
            "str_builder_free",
            "by_url_new",
            "by_url_parse",
            "by_url_free",
            "by_url_to_str",
            "by_url_get_scheme",
            "by_url_get_host",
            "by_url_get_port",
            "by_url_get_path",
            "by_url_get_query",
            "by_url_get_fragment",
            "by_url_get_userinfo",
            "by_url_get_query_value",
            "by_url_set_scheme",
            "by_url_set_host",
            "by_url_set_port",
            "by_url_set_path",
            "by_url_set_query",
            "by_url_encode",
            "by_url_decode",
            "by_dict_contains",
            "by_dict_remove",
            "by_fs_exists",
            "by_fs_is_dir",
            "by_fs_is_file",
            "by_fs_size",
            "by_fs_copy",
            "by_fs_remove",
            "by_fs_create_dir",
            "by_fs_remove_dir",
            "by_fs_rename",
            "by_fs_read_to_string",
            "by_fs_write_string",
            "by_fs_list_dir",
            "by_time_now_ms",
            "by_time_now_us",
            "by_set_new",
            "by_set_free",
            "by_set_add",
            "by_set_contains",
            "by_set_remove",
            "by_set_len",
            "box",
        ];
        for &name in &runtime_fns {
            if let Some(scope) = self.scopes.last_mut() {
                scope.insert(
                    name.to_string(),
                    Symbol::new(
                        name.to_string(),
                        SymKind::Function(FunctionDecl {
                            name: name.to_string(),
                            type_params: Vec::new(),
                            params: Vec::new(),
                            return_type: None,
                            is_async: false,
                            is_const: false,
                            is_static: false,
                            is_abstract: false,
                            is_extern: false,
                            is_test: false,
                            is_bench: false,
                            visibility: Visibility::Public,
                            body: None,
                            span: byc_core::span::Span::dummy(),
                        }),
                    ),
                );
            }
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert(&mut self, name: String, symbol: Symbol) -> Result<(), String> {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, symbol);
            Ok(())
        } else {
            Err("no scope".to_string())
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.get(name) {
                return Some(sym);
            }
        }
        None
    }

    /// Return every class-name registered in the global (bottom) scope.
    /// Used by the visibility check to scan for private members.
    pub fn all_top_level_names(&self) -> Vec<String> {
        self.scopes
            .first()
            .map(|scope| scope.keys().cloned().collect())
            .unwrap_or_default()
    }

    pub fn lookup_type(&self, name: &str) -> Option<Type> {
        if let Some(ty) = self.type_defs.get(name) {
            return Some(ty.clone());
        }
        if let Some(sym) = self.lookup(name) {
            match &sym.kind {
                SymKind::Class(c) => return Some(Type::Named(c.name.clone())),
                SymKind::Struct(s) => return Some(Type::Named(s.name.clone())),
                SymKind::Enum(e) => return Some(Type::Named(e.name.clone())),
                SymKind::TypeAlias(t) => return Some(Type::Named(t.name.clone())),
                SymKind::TypeParam => return Some(Type::TypeVar(0)),
                _ => {}
            }
        }
        None
    }
}
