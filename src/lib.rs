#![deny(warnings)]

pub mod model;
pub mod meta;
pub mod error;
pub mod dims;
pub mod registry;

pub use model::{
    Family, Kind, Spin, DerivativeOrder, FunctionalId, FunctionalFlags,
    HybridType, HybridTermKind, Dimensionality, Thresholds,
};
pub use meta::{FunctionalMeta, Reference, ExtParamSpec, HybridTerm};
pub use error::LibxcRsError;
pub use dims::Dimensions;
pub use registry::{lookup_by_id, lookup_by_name, functional_count, version, version_string};
