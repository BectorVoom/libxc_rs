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

    #[error("workspace mismatch: expected np={expected_np} spin={expected_spin:?}, got np={actual_np} spin={actual_spin:?}")]
    WorkspaceMismatch {
        expected_np: usize,
        actual_np: usize,
        expected_spin: Spin,
        actual_spin: Spin,
    },

    #[error("kernel launch failed: {reason}")]
    KernelLaunchFailed { reason: String },

    #[error("functional {id} is not yet supported by libxc_rs: {reason}")]
    UnsupportedFunctional {
        id: FunctionalId,
        reason: &'static str,
    },

    #[error("external parameter index {index} out of range for functional {id} (has {count} params)")]
    ExtParamIndexOutOfRange {
        id: FunctionalId,
        index: usize,
        count: usize,
    },

    #[error("unknown external parameter name '{name}' for functional {id}")]
    UnknownExtParamName {
        id: FunctionalId,
        name: String,
    },

    #[error("failed to initialize auxiliary functional {aux_id} for parent {parent_id}: {source}")]
    AuxiliaryInitFailed {
        parent_id: FunctionalId,
        aux_id: FunctionalId,
        #[source]
        source: Box<LibxcRsError>,
    },

    #[error("propagation conflict for functional {id}: parent param '{parent_name}' targets aux slot {aux_slot} param '{aux_name}' which is not present")]
    PropagationConflict {
        id: FunctionalId,
        parent_name: &'static str,
        aux_slot: u8,
        aux_name: &'static str,
    },

    // ── Phase 6 plan 06-01 Task 1: new error variants ────────────────

    /// Caller passed `np` exceeding `BatchEvaluator`'s fixed capacity.
    /// Per CONTEXT D-A2-2: no amortized growth — preserves the zero-alloc
    /// invariant of the evaluation hot path. Caller must size for worst case.
    #[error("batch overflow: requested {requested} points exceeds capacity {capacity}")]
    BatchOverflow { requested: usize, capacity: usize },

    /// Caller invoked an operation on an `xc_func_type*` that has not yet
    /// been initialized via `xc_func_init` (or has been reset by `xc_func_end`).
    /// Surfaces from the C ABI; not raised by pure-Rust API paths.
    #[error("uninitialized xc_func_type handle: call xc_func_init before use")]
    UninitializedHandle,

    /// Internal panic was caught at the FFI boundary by `catch_unwind`.
    /// The captured payload (if String / &str) is preserved in `message`;
    /// other payload types yield a generic message. Per CONTEXT D-A4-2.
    #[error("panic in libxc_rs compat layer: {message}")]
    Panicked { message: String },

    /// Caller passed an out-of-range `nspin` to `xc_func_init` (or any other
    /// path that takes a raw `int` spin value). Valid values are 1 (unpolarized)
    /// and 2 (polarized). Used by `compat::raw_handle::xc_func_init` in 06-02a.
    #[error("invalid spin value {0}: expected 1 (unpolarized) or 2 (polarized)")]
    InvalidSpin(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_id_display() {
        let err = LibxcRsError::UnknownFunctionalId(999);
        let msg = format!("{err}");
        assert!(msg.contains("unknown functional ID: 999"), "got: {msg}");
    }

    #[test]
    fn test_removed_id_display() {
        let err = LibxcRsError::RemovedFunctionalId {
            removed_id: 104,
            replacement_id: 1,
            replacement_name: "XC_LDA_X",
        };
        let msg = format!("{err}");
        assert!(msg.contains("removed functional ID 104"), "got: {msg}");
        assert!(msg.contains("XC_LDA_X"), "got: {msg}");
    }

    #[test]
    fn test_unknown_name_display() {
        let err = LibxcRsError::UnknownFunctionalName("bogus".into());
        let msg = format!("{err}");
        assert!(msg.contains("bogus"), "got: {msg}");
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<LibxcRsError>();
    }

    #[test]
    fn test_unsupported_functional_display() {
        let id = FunctionalId::from_raw(1).unwrap();
        let err = LibxcRsError::UnsupportedFunctional {
            id,
            reason: "LDA functional is tracked as deferred",
        };
        let msg = format!("{err}");
        assert!(
            msg.contains("functional 1 is not yet supported"),
            "got: {msg}"
        );
        assert!(msg.contains("deferred"), "got: {msg}");
    }

    #[test]
    fn test_ext_param_index_out_of_range_display() {
        let id = FunctionalId::from_raw(1).unwrap();
        let err = LibxcRsError::ExtParamIndexOutOfRange { id, index: 5, count: 2 };
        let msg = format!("{err}");
        assert!(msg.contains("index 5 out of range"), "got: {msg}");
        assert!(msg.contains("has 2 params"), "got: {msg}");
    }

    #[test]
    fn test_unknown_ext_param_name_display() {
        let id = FunctionalId::from_raw(1).unwrap();
        let err = LibxcRsError::UnknownExtParamName {
            id,
            name: "bogus_param".into(),
        };
        let msg = format!("{err}");
        assert!(msg.contains("'bogus_param'"), "got: {msg}");
        assert!(msg.contains("functional 1"), "got: {msg}");
    }

    #[test]
    fn test_auxiliary_init_failed_display() {
        let parent_id = FunctionalId::from_raw(1).unwrap();
        let aux_id = FunctionalId::from_raw(7).unwrap();
        let inner = LibxcRsError::UnknownFunctionalId(999);
        let err = LibxcRsError::AuxiliaryInitFailed {
            parent_id,
            aux_id,
            source: Box::new(inner),
        };
        let msg = format!("{err}");
        assert!(msg.contains("auxiliary functional 7"), "got: {msg}");
        assert!(msg.contains("parent 1"), "got: {msg}");
    }

    #[test]
    fn test_propagation_conflict_display() {
        let id = FunctionalId::from_raw(1).unwrap();
        let err = LibxcRsError::PropagationConflict {
            id,
            parent_name: "_omega",
            aux_slot: 1,
            aux_name: "_omega",
        };
        let msg = format!("{err}");
        assert!(msg.contains("propagation conflict"), "got: {msg}");
        assert!(msg.contains("'_omega'"), "got: {msg}");
        assert!(msg.contains("aux slot 1"), "got: {msg}");
    }

    // Phase 6 plan 06-01 Task 1: new error variants

    #[test]
    fn batch_overflow_display() {
        let e = LibxcRsError::BatchOverflow { requested: 1024, capacity: 256 };
        let msg = e.to_string();
        assert!(msg.contains("1024"), "got: {msg}");
        assert!(msg.contains("256"), "got: {msg}");
        assert!(msg.to_ascii_lowercase().contains("overflow"), "got: {msg}");
    }

    #[test]
    fn uninitialized_handle_display() {
        let e = LibxcRsError::UninitializedHandle;
        let msg = e.to_string();
        assert!(msg.to_ascii_lowercase().contains("uninitialized"), "got: {msg}");
        assert!(msg.contains("xc_func_init"), "got: {msg}");
    }

    #[test]
    fn panicked_display() {
        let e = LibxcRsError::Panicked { message: "boom".to_string() };
        let msg = e.to_string();
        assert!(msg.to_ascii_lowercase().contains("panic"), "got: {msg}");
        assert!(msg.contains("boom"), "got: {msg}");
    }

    #[test]
    fn invalid_spin_display() {
        let e = LibxcRsError::InvalidSpin(7);
        let msg = e.to_string();
        assert!(msg.contains("7"), "got: {msg}");
        assert!(msg.to_ascii_lowercase().contains("spin"), "got: {msg}");
    }

    #[test]
    fn new_variants_implement_debug() {
        let _ = format!("{:?}", LibxcRsError::UninitializedHandle);
        let _ = format!("{:?}", LibxcRsError::BatchOverflow { requested: 1, capacity: 0 });
        let _ = format!("{:?}", LibxcRsError::Panicked { message: "x".into() });
        let _ = format!("{:?}", LibxcRsError::InvalidSpin(0));
    }
}
