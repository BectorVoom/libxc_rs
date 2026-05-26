//! `FunctionalParams` trait — runtime carrier for per-functional ext_params.
//!
//! `Functional` stores `Box<dyn FunctionalParams + Send + Sync>` (D-13). Dispatch
//! arms downcast via `as_any().downcast_ref::<ConcreteParams>()` to recover the
//! concrete type and extract scalar args for the kernel launch.
//!
//! Functionals with no ext_params use the blanket `NoParams` impl — no per-instance
//! allocation cost beyond the `Box<NoParams>` (zero-sized type, optimal layout).

use std::any::Any;

use libxc_core::error::LibxcRsError;
use libxc_core::model::FunctionalId;

/// Trait carrying per-functional ext_params (raw + derived) through dispatch.
///
/// `Send + Sync` is mandatory (D-13) so `Functional: Send + Sync` auto-derives.
pub trait FunctionalParams: Send + Sync {
    /// Number of ext_params declared by the functional's metadata.
    fn ext_param_count(&self) -> usize;

    /// Current raw ext_params values.
    fn raw_ext_params(&self) -> &[f64];

    /// Bulk set; validates length equals `ext_param_count`.
    /// May recompute derived fields (CAM alpha/beta/omega, etc.).
    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError>;

    /// Escape hatch for dispatch arms to downcast to concrete impl.
    fn as_any(&self) -> &dyn Any;
}

/// Zero-ext_params blanket impl. Used by the majority of functionals.
///
/// Zero-sized type: no per-instance allocation cost beyond the `Box<dyn ...>`
/// vtable pointer.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoParams;

impl FunctionalParams for NoParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(0),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_params_count_is_zero() {
        let p = NoParams;
        assert_eq!(p.ext_param_count(), 0);
        assert!(p.raw_ext_params().is_empty());
    }

    #[test]
    fn no_params_set_empty_ok() {
        let mut p = NoParams;
        assert!(p.set_ext_params(&[]).is_ok());
    }

    #[test]
    fn no_params_set_nonempty_errors() {
        let mut p = NoParams;
        let result = p.set_ext_params(&[1.0]);
        match result {
            Err(LibxcRsError::ExtParamCountMismatch { expected, actual, .. }) => {
                assert_eq!(expected, 0);
                assert_eq!(actual, 1);
            }
            other => panic!("expected ExtParamCountMismatch, got {other:?}"),
        }
    }

    #[test]
    fn no_params_as_any_downcasts() {
        let p: Box<dyn FunctionalParams> = Box::new(NoParams);
        let downcast = p.as_any().downcast_ref::<NoParams>();
        assert!(downcast.is_some());
    }

    #[test]
    fn no_params_wrong_type_downcast_returns_none() {
        let p: Box<dyn FunctionalParams> = Box::new(NoParams);
        // Some other ZST; should not match NoParams.
        struct OtherType;
        let downcast = p.as_any().downcast_ref::<OtherType>();
        assert!(downcast.is_none());
    }

    #[test]
    fn no_params_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<NoParams>();
        assert_send_sync::<Box<dyn FunctionalParams>>();
    }
}
