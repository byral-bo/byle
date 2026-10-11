#![allow(clippy::collapsible_match, clippy::doc_lazy_continuation)]

pub mod analyzer;
pub mod contracts;
pub mod module_resolver;
pub mod scope;
pub mod symbol_table;
pub mod type_checker;

pub use analyzer::SemanticAnalyzer;
pub use module_resolver::ModuleResolver;
