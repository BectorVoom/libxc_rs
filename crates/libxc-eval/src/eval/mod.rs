// The per-family CubeCL dispatch modules were deleted with the archived kernel
// tree. What is left are the stubs that already stood in for a family whose
// kernels were not compiled: they return `UnsupportedFunctional`, which is now
// simply always the answer here. Numerical dispatch lives in `libxc-reval`.
pub mod mix;
pub mod workspace;

pub fn dispatch_lda(
    functional: libxc_core::model::LdaFunctional,
    _input: &libxc_core::input::LdaInput,
    _order: libxc_core::model::DerivativeOrder,
    _output: &mut libxc_core::output::LdaOutput,
    _params: &dyn crate::functional::FunctionalParams,
    _thresholds: &libxc_core::model::Thresholds,
) -> Result<(), libxc_core::error::LibxcRsError> {
    Err(libxc_core::error::LibxcRsError::UnsupportedFunctional {
        id: functional.to_id(),
        reason: "LDA family not compiled in this build (enable feature `oracle-lda`)",
    })
}
pub fn dispatch_gga(
    functional: libxc_core::model::GgaFunctional,
    _input: &libxc_core::input::GgaInput,
    _order: libxc_core::model::DerivativeOrder,
    _output: &mut libxc_core::output::GgaOutput,
    _params: &dyn crate::functional::FunctionalParams,
    _thresholds: &libxc_core::model::Thresholds,
) -> Result<(), libxc_core::error::LibxcRsError> {
    Err(libxc_core::error::LibxcRsError::UnsupportedFunctional {
        id: functional.to_id(),
        reason: "GGA family not compiled in this build (enable feature `oracle-gga`)",
    })
}
pub fn dispatch_mgga(
    functional: libxc_core::model::MggaFunctional,
    _input: &libxc_core::input::MggaInput,
    _order: libxc_core::model::DerivativeOrder,
    _output: &mut libxc_core::output::MggaOutput,
    _params: &dyn crate::functional::FunctionalParams,
    _thresholds: &libxc_core::model::Thresholds,
) -> Result<(), libxc_core::error::LibxcRsError> {
    Err(libxc_core::error::LibxcRsError::UnsupportedFunctional {
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
