//! libxc_rs workspace root documentation derived from docs/libxc_rs_detailed_design.md.
#![doc = include_str!("../docs/libxc_rs_detailed_design.md")]

pub mod api;
pub mod compat;
pub mod error;
pub mod eval;
pub mod generated;
pub mod input;
pub mod kernel;
pub mod layout;
pub mod meta;
pub mod model;
pub mod output;
pub mod registry;
pub mod runtime;
pub mod workspace;
