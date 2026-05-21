//! Standalone G3 canary verification crate (Phase 11.1 Plan 03).
//!
//! Intentionally empty — all logic lives in `tests/`. This crate exists so the
//! `mgga_c_b94` direct-call parity test can be built WITHOUT pulling the
//! `libxc_rs` umbrella crate (which would force all 281 kernels to compile).
//! See Cargo.toml for the full rationale.
