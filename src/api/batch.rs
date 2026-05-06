//! Workspace-only batch evaluation driver.
//!
//! [`BatchEvaluator`] owns one [`EvaluationWorkspace`] sized at
//! construction for `np_max` grid points. Caller passes `&Functional`,
//! `&Input`, `&mut Output` on every `evaluate` call. Per CONTEXT D-A2-1.
//!
//! Layer-3 module — zero `unsafe`.

use crate::api::evaluate::EvaluateInput;
use crate::error::LibxcRsError;
use crate::eval::workspace::EvaluationWorkspace;
use crate::functional::Functional;
use crate::model::{DerivativeOrder, Spin};

/// Reusable workspace for repeated `Functional::evaluate_*` calls on
/// the same grid. Construct once, evaluate many times.
///
/// `np_max` is fixed at construction (no amortized growth — see
/// CONTEXT D-A2-2). Calls with `input.np() > np_max` return
/// [`LibxcRsError::BatchOverflow`].
///
/// The workspace is sized for the MGGA superset (Phase 3 D-12), so a
/// single `BatchEvaluator` can drive any LDA/GGA/MGGA functional with
/// the same `(spin, np_max)`.
///
/// # Threading
/// Callers may share a `BatchEvaluator` across threads only behind a
/// `Mutex`/`RwLock`: `evaluate` takes `&mut self` because the underlying
/// workspace is mutated during evaluation. The type itself is
/// `Send + Sync`.
pub struct BatchEvaluator {
    ws: EvaluationWorkspace,
    np_max: usize,
    spin: Spin,
}

impl BatchEvaluator {
    /// Allocate workspace for up to `np_max` points at the given spin.
    ///
    /// The workspace is sized for the MGGA superset (Phase 3 D-12), so a
    /// single `BatchEvaluator` can drive any LDA/GGA/MGGA functional
    /// with the same `(spin, np_max)`.
    pub fn new(spin: Spin, np_max: usize) -> Self {
        Self {
            ws: EvaluationWorkspace::new(np_max, spin),
            np_max,
            spin,
        }
    }

    /// Auto-dispatching evaluation entry point. Family is selected by
    /// the input type at compile time via the sealed [`EvaluateInput`]
    /// trait.
    ///
    /// # Errors
    /// - [`LibxcRsError::BatchOverflow`] — `input.np() > np_max`.
    /// - [`LibxcRsError::SpinMismatch`] — `functional.spin() != self.spin`.
    /// - [`LibxcRsError::FamilyMismatch`] — input's family does not match
    ///   the functional's family.
    /// - Any error from the underlying `Functional::evaluate_*` call.
    pub fn evaluate<I: EvaluateInput>(
        &mut self,
        functional: &Functional,
        input: &I,
        order: DerivativeOrder,
        output: &mut I::Output<'_>,
    ) -> Result<(), LibxcRsError> {
        let np = input.np();
        if np > self.np_max {
            return Err(LibxcRsError::BatchOverflow {
                requested: np,
                capacity: self.np_max,
            });
        }
        if functional.spin() != self.spin {
            return Err(LibxcRsError::SpinMismatch {
                expected: self.spin,
                actual: functional.spin(),
            });
        }
        input.dispatch(functional, order, output, &mut self.ws)
    }

    /// Internal accessor used by tests to verify "no realloc across N
    /// evaluations" — returns the workspace's grid-point capacity.
    #[cfg(test)]
    pub(crate) fn workspace_np_for_test(&self) -> usize {
        self.ws.np()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::LdaInput;
    use crate::model::{DerivativeOrder, FunctionalId, Spin};
    use crate::output::LdaOutput;

    /// Test 4: passing `np > np_max` returns [`LibxcRsError::BatchOverflow`]
    /// rather than panicking or silently truncating.
    #[test]
    fn overflow_returns_error() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();

        // BatchEvaluator sized for 4 points, but input has 8.
        let np = 8;
        let rho = vec![0.1_f64; np];
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0_f64; np];
        let mut out = LdaOutput::new(
            Some(&mut zk),
            None,
            None,
            None,
            None,
            np,
            Spin::Unpolarized,
        )
        .unwrap();

        let mut be = BatchEvaluator::new(Spin::Unpolarized, 4);
        let err = be
            .evaluate(&f, &input, DerivativeOrder::Exc, &mut out)
            .unwrap_err();
        match err {
            LibxcRsError::BatchOverflow { requested, capacity } => {
                assert_eq!(requested, 8);
                assert_eq!(capacity, 4);
            }
            other => panic!("expected BatchOverflow, got {other:?}"),
        }
    }

    /// Test 5: looping `evaluate` 100 times on the same BatchEvaluator
    /// must not cause workspace reallocation (`workspace_np_for_test`
    /// reports the same capacity at start and end).
    #[test]
    fn workspace_reuse_no_realloc() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();

        let np = 4;
        let rho = vec![0.1_f64, 0.2, 0.3, 0.4];
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        let mut be = BatchEvaluator::new(Spin::Unpolarized, np);
        let initial_capacity = be.workspace_np_for_test();

        for _ in 0..100 {
            let mut zk = vec![0.0_f64; np];
            let mut out = LdaOutput::new(
                Some(&mut zk),
                None,
                None,
                None,
                None,
                np,
                Spin::Unpolarized,
            )
            .unwrap();
            be.evaluate(&f, &input, DerivativeOrder::Exc, &mut out)
                .unwrap();
        }

        let final_capacity = be.workspace_np_for_test();
        assert_eq!(
            initial_capacity, final_capacity,
            "workspace must not realloc"
        );
    }

    /// Test 6: BatchEvaluator implements `Send + Sync` (compile-only).
    /// Mirrors the pattern at `src/functional/mod.rs::thread_safety_tests`.
    #[test]
    fn batch_evaluator_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<BatchEvaluator>();
    }
}
