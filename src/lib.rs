#![deny(warnings)]
// CubeCL #[cube] macro expansion generates code that triggers these lints.
// The excessive_precision lint is also inappropriate for scientific constants
// where trailing digits are intentional for documentation clarity.
#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

// Data layer moved to libxc-core (Phase 10); re-exported so sibling root
// modules and downstream `crate::model::`/`crate::meta::` paths keep resolving.
pub use libxc_core::{model, meta, error, dims, registry, input, output};
pub use libxc_core::deferred;

// Orchestration layer moved to libxc-eval (Phase 10); re-exported so api/compat
// and downstream `crate::eval::`/`crate::kernel::`/`libxc_rs::math` keep resolving.
pub use libxc_eval::{eval, functional, kernel};
pub use libxc_eval::math;

pub mod api;
pub mod compat;

pub use model::{
    Family, Kind, Spin, DerivativeOrder, FunctionalId, FunctionalFlags,
    HybridType, HybridTermKind, Dimensionality, Thresholds,
    LdaFunctional, GgaFunctional, MggaFunctional,
};
pub use meta::{FunctionalMeta, Reference, ExtParamSpec, HybridTerm};
pub use error::LibxcRsError;
pub use dims::Dimensions;
pub use registry::{lookup_by_id, lookup_by_name, functional_count, version, version_string};
pub use input::{LdaInput, GgaInput, MggaInput};
pub use output::{LdaOutput, GgaOutput, MggaOutput, OutputMask};
pub use eval::{dispatch_lda, dispatch_gga, dispatch_mgga};
pub use functional::{
    classify_hybrid, CamCoefficients, Functional, FunctionalParams, NlcCoefficients, NoParams,
};
pub use api::{BatchEvaluator, EvaluateInput, FunctionalBuilder};
// Opaque C-ABI handle types (compat layer). Re-exported at the crate root so
// Rust-side callers and the include/xc.h generator can name them directly.
pub use compat::c_layout::{xc_func_info_type, xc_func_type};
