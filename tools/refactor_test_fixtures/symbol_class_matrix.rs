//! D-22 Gate 1 fixture: symbol class coverage matrix for the cast_from policy.
//!
//! Per CONTEXT.md D-22, this file covers every known symbol class the
//! cast_from-aware extension of `tools/refactor_helpers_generic.py` must
//! classify correctly. Running the script on this file MUST produce a diff
//! where every change matches the per-class policy from D-20.
//!
//! Classes covered:
//!  1. f64 const declaration + usage in generic body          -> F::cast_from(IDENT)
//!  2. f32 const declaration + usage in generic body          -> F::new(IDENT) keep
//!  3. Doc-comment with constant-like text (LDA, MGGA, ID)    -> revert (bare ident in comment text)
//!  4. String literal with constant-like text ("BR89", "17.5K") -> revert (bare ident in string)
//!  5. Range operator `..` (for _ in 0..500)                  -> preserve (no F::new wrap)
//!  6. `_f64` literal suffix (3.0_f64)                        -> revert / repair to F::new(3.0)
//!  7. Double-wrap pattern (F::F::new(MAX) restoring f64::MAX)-> repair to F::cast_from(f64::MAX)
//!  8. Non-generic helper context (pub fn is_deferred ...)    -> revert all F::new wraps
//!  9. Mixed: f64 const used in arithmetic against F          -> F::cast_from(IDENT) + F arithmetic
//!
//! This fixture is a transformation TARGET (the script writes to it). It is
//! NOT a workspace member; standalone rustc compile is OPTIONAL — Gate 1
//! acceptance is the diff matching per-class policy, not standalone compile.

use cubecl::prelude::*;

// Class 1 — f64 const decl
const SQRT_DBL_EPSILON: f64 = 1.4901161193847656e-8;
const LOG_DBL_MAX: f64 = 709.7827128933840;

// Class 2 — f32 const decl
const F32_TINY: f32 = 1.0e-6;

// Classes 1, 2, 9 — generic body using both
/// Class 3 — doc-comment with constant-like text: LDA functional helper.
/// Implements the LDA exchange formula with normalized prefactor ID 1.
#[cube]
pub fn fixture_generic<F: Float>(x: F) -> F {
    // POST-SCRIPT-EXPECT: F::cast_from(SQRT_DBL_EPSILON) — class 1
    let eps = F::new(SQRT_DBL_EPSILON);
    // POST-SCRIPT-EXPECT: F::cast_from(LOG_DBL_MAX) — class 1
    let cap = F::new(LOG_DBL_MAX);
    // POST-SCRIPT-EXPECT: F::new(F32_TINY) preserved — class 2
    let tiny = F::new(F32_TINY);
    // POST-SCRIPT-EXPECT: F::new(2.0) preserved (numeric literal) — orthogonal class
    let two = F::new(2.0);
    // Class 9 — mixed arithmetic
    if x < eps { tiny } else if x > cap { two } else { x }
}

// Class 5 — range operator with potential mis-wrap source
pub fn fixture_range_loop() -> u32 {
    let mut total: u32 = 0;
    // POST-SCRIPT-EXPECT: for _ in 0..500 — class 5 (script must NOT wrap this)
    for _ in 0..500 {
        total += 1;
    }
    total
}

// Class 6 — _f64 literal suffix (input the script must REPAIR if mis-wrapped)
pub fn fixture_f64_suffix() -> f64 {
    // POST-SCRIPT-EXPECT: 3.0_f64 preserved (this is valid Rust) — class 6
    let x: f64 = 3.0_f64;
    x
}

// Class 7 — double-wrap pattern from special.rs:224
#[cube]
pub fn fixture_double_wrap<F: Float>() -> F {
    // POST-SCRIPT-EXPECT: F::cast_from(f64::MAX) — class 7 (after D-23 surgical fix)
    let max_val = F::cast_from(f64::MAX);
    max_val
}

// Class 8 — non-generic helper context
pub fn fixture_is_deferred(id: u16) -> bool {
    // POST-SCRIPT-EXPECT: no F::new wraps — class 8 (non-generic, revert all)
    matches!(id, 1 | 2 | 3)
}

// Class 4 — string literals with constant-like text
pub fn fixture_string_literals() -> &'static str {
    // POST-SCRIPT-EXPECT: "17.5K BR89 model" preserved — class 4 (revert all string corruptions)
    "17.5K BR89 model"
}
