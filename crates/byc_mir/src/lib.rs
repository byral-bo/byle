// byc_mir — Mid-level IR and optimization

// Retained analysis/lowering helpers are extension points for the MIR pipeline.
#![allow(
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::enum_variant_names,
    clippy::extra_unused_lifetimes,
    clippy::if_same_then_else,
    clippy::match_single_binding,
    clippy::needless_range_loop,
    clippy::redundant_guards,
    clippy::too_many_arguments,
    clippy::vec_box,
    dead_code
)]
//
// Depends on: byc_core
//
// Responsibilities:
//   - MIR definition
//   - AST to MIR lowering
//   - Optimization passes
//   - Control flow analysis

pub mod borrow_analysis;
pub mod lower;
pub mod mir;
pub mod optimize;
pub mod ssa;
