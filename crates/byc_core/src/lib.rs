// byc_core — Foundation types used by all compiler crates
//
// This crate contains no dependencies on other BYLE crates.
// It defines:
//   - AST node definitions
//   - Span (source location)
//   - SourceMap
//   - Symbol IDs
//   - Diagnostic types
//   - Type representations

#![allow(dead_code)]

pub mod ast;
pub mod diagnostic;
pub mod resolver;
pub mod semver;
pub mod source_map;
pub mod span;
pub mod symbol;
pub mod types;
