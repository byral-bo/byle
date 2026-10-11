// byc_tools — Developer tooling

#![allow(
    clippy::if_same_then_else,
    clippy::only_used_in_recursion,
    clippy::should_implement_trait
)]
//
// Depends on: byc_core, byc_frontend
//
// Responsibilities:
//   - LSP (Language Server Protocol) implementation
//   - Code formatter (by fmt)
//   - Code completion
//   - Diagnostics
//   - Refactoring tools

pub mod completion;
pub mod formatter;
pub mod lsp;
pub mod package;
