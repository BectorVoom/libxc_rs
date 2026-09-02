pub mod mix;
pub mod workspace;

pub fn dispatch_lda(
    functional: libxc_core::model::LdaFunctional,
    input: &libxc_core::input::LdaInput,
    order: libxc_core::model::DerivativeOrder,
    output: &mut libxc_core::output::LdaOutput,
    _params: &dyn crate::functional::FunctionalParams,
    thresholds: &libxc_core::model::Thresholds,
) -> Result<(), libxc_core::error::LibxcRsError> {
    libxc_reval::routing::dispatch_lda(functional, input, output, order, input.spin(), thresholds)
}

pub fn dispatch_lda_by_id(
    id: libxc_core::model::FunctionalId,
    input: &libxc_core::input::LdaInput,
    order: libxc_core::model::DerivativeOrder,
    output: &mut libxc_core::output::LdaOutput,
    ext: Option<&[f64]>,
    thresholds: &libxc_core::model::Thresholds,
) -> Result<(), libxc_core::error::LibxcRsError> {
    libxc_reval::routing::dispatch_lda_by_id_with(
        id, input, output, order, input.spin(), thresholds, ext,
    )
}

pub fn dispatch_gga(
    functional: libxc_core::model::GgaFunctional,
    input: &libxc_core::input::GgaInput,
    order: libxc_core::model::DerivativeOrder,
    output: &mut libxc_core::output::GgaOutput,
    _params: &dyn crate::functional::FunctionalParams,
    thresholds: &libxc_core::model::Thresholds,
) -> Result<(), libxc_core::error::LibxcRsError> {
    libxc_reval::routing::dispatch_gga(functional, input, output, order, input.spin(), thresholds)
}

pub fn dispatch_gga_by_id(
    id: libxc_core::model::FunctionalId,
    input: &libxc_core::input::GgaInput,
    order: libxc_core::model::DerivativeOrder,
    output: &mut libxc_core::output::GgaOutput,
    ext: Option<&[f64]>,
    thresholds: &libxc_core::model::Thresholds,
) -> Result<(), libxc_core::error::LibxcRsError> {
    libxc_reval::routing::dispatch_gga_by_id_with(
        id, input, output, order, input.spin(), thresholds, ext,
    )
}

pub fn dispatch_mgga(
    functional: libxc_core::model::MggaFunctional,
    input: &libxc_core::input::MggaInput,
    order: libxc_core::model::DerivativeOrder,
    output: &mut libxc_core::output::MggaOutput,
    _params: &dyn crate::functional::FunctionalParams,
    thresholds: &libxc_core::model::Thresholds,
) -> Result<(), libxc_core::error::LibxcRsError> {
    libxc_reval::routing::dispatch_mgga(functional, input, output, order, input.spin(), thresholds)
}

pub fn dispatch_mgga_by_id(
    id: libxc_core::model::FunctionalId,
    input: &libxc_core::input::MggaInput,
    order: libxc_core::model::DerivativeOrder,
    output: &mut libxc_core::output::MggaOutput,
    ext: Option<&[f64]>,
    thresholds: &libxc_core::model::Thresholds,
) -> Result<(), libxc_core::error::LibxcRsError> {
    libxc_reval::routing::dispatch_mgga_by_id_with(
        id, input, output, order, input.spin(), thresholds, ext,
    )
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
