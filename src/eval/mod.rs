pub mod dispatch;
pub mod gga_dispatch;
pub mod mgga_dispatch;
pub mod mix;
pub mod workspace;
pub use dispatch::dispatch_lda;
pub use gga_dispatch::dispatch_gga;
pub use mgga_dispatch::dispatch_mgga;
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
