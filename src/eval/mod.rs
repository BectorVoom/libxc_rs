pub mod dispatch;
pub mod mix;
pub mod workspace;
pub use dispatch::{dispatch_lda, LdaFunctionalParams};
pub use mix::{add_to_mix, evaluate_mixed_lda, AuxiliaryConfig};
pub use workspace::EvaluationWorkspace;
