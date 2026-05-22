// Phase 11 / 11-12 (G-2, Path A): the per-family dispatch modules are gated
// behind per-family features (their kernel refs resolve only when the family's
// kernels are compiled). default = all three, so the full umbrella build is
// unchanged. When a family is off, a stub `dispatch_<fam>` below keeps the
// symbol defined so evaluate.rs / mix.rs / lib.rs need no cfg cascade.
#[cfg(feature = "oracle-lda")]
pub mod dispatch;
#[cfg(feature = "oracle-gga")]
pub mod gga_dispatch;
#[cfg(feature = "oracle-mgga")]
pub mod mgga_dispatch;
pub mod mix;
pub mod workspace;
#[cfg(feature = "oracle-lda")]
pub use dispatch::dispatch_lda;
#[cfg(feature = "oracle-gga")]
pub use gga_dispatch::dispatch_gga;
#[cfg(feature = "oracle-mgga")]
pub use mgga_dispatch::dispatch_mgga;

// Stubs for families not compiled into this build. Same signature as the real
// dispatch fns; return UnsupportedFunctional (correct — that family's kernels
// are absent). Keeps `crate::eval::dispatch_<fam>` always resolvable.
#[cfg(not(feature = "oracle-lda"))]
pub fn dispatch_lda(
    functional: crate::model::LdaFunctional,
    _input: &crate::input::LdaInput,
    _order: crate::model::DerivativeOrder,
    _output: &mut crate::output::LdaOutput,
    _params: &dyn crate::functional::FunctionalParams,
    _thresholds: &crate::model::Thresholds,
) -> Result<(), crate::error::LibxcRsError> {
    Err(crate::error::LibxcRsError::UnsupportedFunctional {
        id: functional.to_id(),
        reason: "LDA family not compiled in this build (enable feature `oracle-lda`)",
    })
}
#[cfg(not(feature = "oracle-gga"))]
pub fn dispatch_gga(
    functional: crate::model::GgaFunctional,
    _input: &crate::input::GgaInput,
    _order: crate::model::DerivativeOrder,
    _output: &mut crate::output::GgaOutput,
    _params: &dyn crate::functional::FunctionalParams,
    _thresholds: &crate::model::Thresholds,
) -> Result<(), crate::error::LibxcRsError> {
    Err(crate::error::LibxcRsError::UnsupportedFunctional {
        id: functional.to_id(),
        reason: "GGA family not compiled in this build (enable feature `oracle-gga`)",
    })
}
#[cfg(not(feature = "oracle-mgga"))]
pub fn dispatch_mgga(
    functional: crate::model::MggaFunctional,
    _input: &crate::input::MggaInput,
    _order: crate::model::DerivativeOrder,
    _output: &mut crate::output::MggaOutput,
    _params: &dyn crate::functional::FunctionalParams,
    _thresholds: &crate::model::Thresholds,
) -> Result<(), crate::error::LibxcRsError> {
    Err(crate::error::LibxcRsError::UnsupportedFunctional {
        id: functional.to_id(),
        reason: "MGGA family not compiled in this build (enable feature `oracle-mgga`)",
    })
}
pub use mix::{
    add_to_mix, evaluate_mixed_gga, evaluate_mixed_lda, evaluate_mixed_lda_functional,
    evaluate_mixed_mgga, AuxiliaryConfig,
};
pub use workspace::EvaluationWorkspace;

// Alias kept for backward compat with verify/tests/lda_oracle.rs and other
// external callers that imported `LdaFunctionalParams` from the old
// dispatch module. New code should reference `LdaXParams` directly from
// `crate::functional::params_lda`.
pub use crate::functional::params_lda::LdaXParams as LdaFunctionalParams;
