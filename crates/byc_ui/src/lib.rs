// UI generation is paused but kept as a compatibility subsystem.
#![allow(
    clippy::if_same_then_else,
    clippy::manual_strip,
    clippy::no_effect_replace,
    clippy::should_implement_trait,
    clippy::too_many_arguments,
    dead_code,
    unused_assignments,
    unused_variables,
    unused_mut
)]

pub mod app_config;
pub mod ast;
pub mod backend;
pub mod embedded_runtime;
pub mod ir;
pub mod parser;
pub mod resolver;

// Legacy modules (will be removed in future — functionality migrated to backend::web)
pub mod anim_gen;
pub mod js_gen;
pub mod style_gen;
