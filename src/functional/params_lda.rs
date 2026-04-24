//! Per-functional `FunctionalParams` impls for LDA functionals.
//!
//! Plan 05-02 scope: hand-wired concrete impl for `LdaXParams` (the only LDA
//! functional whose dispatch arm currently consumes a runtime ext_param --
//! `alpha`, the Slater exchange scaling). All other LDA functionals (the
//! remaining 36 compiled + 4 deferred) reuse `NoParams` via the
//! `construct_params` dispatch table in `lifecycle.rs`.
//!
//! Per-functional structs for the other 36 ext-param-bearing LDA functionals
//! (lda_x_erf, lda_x_yukawa, lda_x_sloc, lda_c_hl, lda_c_pz, lda_c_pw,
//! lda_c_wigner, lda_c_chachiyo*, lda_xc_1d_ehwlrg*, etc.) are tracked as
//! follow-up work: the dispatch arms in `src/eval/dispatch.rs` continue to
//! use hardcoded libxc defaults (matching the C oracle bit-for-bit), so
//! `Functional::new(id, _).set_ext_param("omega", 0.5)` is a Phase 5 follow-up.
//!
//! See `params.rs::FunctionalParams` for the trait shape and `lifecycle.rs::
//! construct_params` for the dispatch table.

use std::any::Any;

use crate::error::LibxcRsError;
use crate::functional::params::{FunctionalParams, NoParams};
use crate::model::FunctionalId;

/// Slater exchange scaling for `lda_x` (XC_LDA_X = id 1). Defaults to 1.0.
///
/// This is the historical `LdaFunctionalParams { alpha }` struct, renamed
/// to follow the per-functional naming convention. A type alias
/// `LdaFunctionalParams = LdaXParams` is exported from `crate::eval` so
/// existing verify tests continue to compile.
#[derive(Debug, Clone)]
pub struct LdaXParams {
    raw: Box<[f64]>,
    pub(crate) alpha: f64,
}

impl LdaXParams {
    /// libxc 7.0.0 default for `lda_x`: alpha = 1.0 (Slater).
    pub fn from_defaults() -> Self {
        let raw = Box::<[f64]>::from([1.0]);
        Self { alpha: raw[0], raw }
    }

    /// Backward-compat constructor matching the old `LdaFunctionalParams::default()`.
    pub fn default() -> Self {
        Self::from_defaults()
    }
}

impl Default for LdaXParams {
    fn default() -> Self {
        Self::from_defaults()
    }
}

impl FunctionalParams for LdaXParams {
    fn ext_param_count(&self) -> usize {
        1
    }

    fn raw_ext_params(&self) -> &[f64] {
        &self.raw
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if vals.len() != 1 {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(1), // XC_LDA_X
                expected: 1,
                actual: vals.len(),
            });
        }
        self.raw = Box::<[f64]>::from(vals);
        self.alpha = vals[0];
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// --- Type aliases for the remaining LDA functionals (Plan 05-02 scope: NoParams) ---
//
// Per Plan 05-02 deferral note (see module-level docs above), the 36 remaining
// LDA functionals + 4 deferred LDA IDs use `NoParams` until per-functional
// derivation work lands in a follow-up plan.

pub type LdaX2dParams = NoParams;
pub type LdaXRelParams = NoParams;
pub type LdaXErfParams = NoParams;
pub type LdaXSlocParams = NoParams;
pub type LdaXYukawaParams = NoParams;
pub type LdaCRpaParams = NoParams;
pub type LdaCHlParams = NoParams;
pub type LdaCVwnParams = NoParams;
pub type LdaCVwnRpaParams = NoParams;
pub type LdaCVwn1Params = NoParams;
pub type LdaCVwn2Params = NoParams;
pub type LdaCVwn3Params = NoParams;
pub type LdaCVwn4Params = NoParams;
pub type LdaCPzParams = NoParams;
pub type LdaCPwParams = NoParams;
pub type LdaCWignerParams = NoParams;
pub type LdaCRc04Params = NoParams;
pub type LdaC2dAmgbParams = NoParams;
pub type LdaC2dPrmParams = NoParams;
pub type LdaC1dCscParams = NoParams;
pub type LdaC1dLoosParams = NoParams;
pub type LdaCGk72Params = NoParams;
pub type LdaCGombasParams = NoParams;
pub type LdaCLp96Params = NoParams;
pub type LdaCMl1Params = NoParams;
pub type LdaCW20Params = NoParams;
pub type LdaCChachiyoParams = NoParams;
pub type LdaCChachiyoModParams = NoParams;
pub type LdaKTfParams = NoParams;
pub type LdaKZlpParams = NoParams;
pub type LdaXcTeter93Params = NoParams;
pub type LdaXcZlpParams = NoParams;
pub type LdaXcTihParams = NoParams;
pub type LdaXc1dEhwlrg1Params = NoParams;
pub type LdaXc1dEhwlrg2Params = NoParams;
pub type LdaXc1dEhwlrg3Params = NoParams;
pub type HybLdaXcBn05Params = NoParams;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lda_x_params_default_alpha_one() {
        let p = LdaXParams::from_defaults();
        assert_eq!(p.alpha, 1.0);
        assert_eq!(p.ext_param_count(), 1);
        assert_eq!(p.raw_ext_params(), &[1.0]);
    }

    #[test]
    fn lda_x_params_set_alpha_writes_through() {
        let mut p = LdaXParams::from_defaults();
        p.set_ext_params(&[0.7]).unwrap();
        assert_eq!(p.alpha, 0.7);
        assert_eq!(p.raw_ext_params(), &[0.7]);
    }

    #[test]
    fn lda_x_params_set_wrong_length_errors() {
        let mut p = LdaXParams::from_defaults();
        let result = p.set_ext_params(&[1.0, 2.0]);
        match result {
            Err(LibxcRsError::ExtParamCountMismatch { expected, actual, .. }) => {
                assert_eq!(expected, 1);
                assert_eq!(actual, 2);
            }
            other => panic!("expected ExtParamCountMismatch, got {other:?}"),
        }
    }

    #[test]
    fn lda_x_params_downcast_succeeds() {
        let p: Box<dyn FunctionalParams> = Box::new(LdaXParams::from_defaults());
        let recovered = p.as_any().downcast_ref::<LdaXParams>();
        assert!(recovered.is_some());
        assert_eq!(recovered.unwrap().alpha, 1.0);
    }

    #[test]
    fn lda_x_params_wrong_type_downcast_returns_none() {
        let p: Box<dyn FunctionalParams> = Box::new(LdaXParams::from_defaults());
        let recovered = p.as_any().downcast_ref::<NoParams>();
        assert!(recovered.is_none());
    }
}
