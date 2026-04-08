use crate::model::{DerivativeOrder, Family, FunctionalId, Spin};

#[derive(Debug, thiserror::Error)]
pub enum LibxcRsError {
    #[error("unknown functional ID: {0}")]
    UnknownFunctionalId(u16),

    #[error("removed functional ID {removed_id}; use {replacement_id} ({replacement_name}) instead")]
    RemovedFunctionalId {
        removed_id: u16,
        replacement_id: u16,
        replacement_name: &'static str,
    },

    #[error("no functional found with name '{0}'")]
    UnknownFunctionalName(String),

    #[error("functional {id} does not support derivative order {order:?} (max: {max:?})")]
    UnsupportedDerivativeOrder {
        id: FunctionalId,
        order: DerivativeOrder,
        max: DerivativeOrder,
    },

    #[error("input buffer '{field}' size mismatch: expected {expected}, got {actual}")]
    InputBufferSizeMismatch {
        field: &'static str,
        expected: usize,
        actual: usize,
    },

    #[error("output buffer '{field}' size mismatch: expected {expected}, got {actual}")]
    OutputBufferSizeMismatch {
        field: &'static str,
        expected: usize,
        actual: usize,
    },

    #[error("family mismatch: functional {id} is {expected:?}, but {actual:?} input provided")]
    FamilyMismatch {
        id: FunctionalId,
        expected: Family,
        actual: Family,
    },

    #[error("spin mode mismatch: functional configured for {expected:?}, input is {actual:?}")]
    SpinMismatch {
        expected: Spin,
        actual: Spin,
    },

    #[error("external parameter '{name}' not found for functional {id}")]
    ExtParamNotFound {
        id: FunctionalId,
        name: String,
    },

    #[error("external parameter count mismatch for {id}: expected {expected}, got {actual}")]
    ExtParamCountMismatch {
        id: FunctionalId,
        expected: usize,
        actual: usize,
    },

    #[error("GPU device not available: {reason}")]
    GpuNotAvailable { reason: String },

    #[error("GPU device '{device}' does not support f64 computation")]
    DeviceCapabilityMismatch { device: String },

    #[error("all {np} input grid points have density below threshold ({threshold})")]
    AllBelowThreshold { np: usize, threshold: f64 },
}
