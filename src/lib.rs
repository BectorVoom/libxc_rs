#![deny(warnings)]
// CubeCL #[cube] macro expansion generates code that triggers these lints.
// The excessive_precision lint is also inappropriate for scientific constants
// where trailing digits are intentional for documentation clarity.
#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

pub mod model;
pub mod meta;
pub mod error;
pub mod dims;
pub mod registry;
pub mod math;
pub mod kernel;
pub mod input;
pub mod output;
pub mod eval;

pub use model::{
    Family, Kind, Spin, DerivativeOrder, FunctionalId, FunctionalFlags,
    HybridType, HybridTermKind, Dimensionality, Thresholds,
};
pub use meta::{FunctionalMeta, Reference, ExtParamSpec, HybridTerm};
pub use error::LibxcRsError;
pub use dims::Dimensions;
pub use registry::{lookup_by_id, lookup_by_name, functional_count, version, version_string};
pub use input::{LdaInput, GgaInput, MggaInput};
pub use output::{LdaOutput, GgaOutput, MggaOutput, OutputMask};
pub use eval::dispatch_lda;
