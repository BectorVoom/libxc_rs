//! Sealed `EvaluateInput` dispatch trait + 3 impls (LdaInput / GgaInput /
//! MggaInput).
//!
//! Each implementation owns the family-specific dispatch call to the
//! Phase-5 `Functional::evaluate_*` method. [`crate::api::BatchEvaluator`]'s
//! `evaluate` is a one-line forward to `input.dispatch(...)`. Per
//! CONTEXT D-A3-1.
//!
//! Layer-3 module — zero `unsafe` (BUILD-04 / COMPAT-03 invariant).

use crate::error::LibxcRsError;
use crate::eval::workspace::EvaluationWorkspace;
use crate::functional::Functional;
use crate::input::{GgaInput, LdaInput, MggaInput};
use crate::model::{DerivativeOrder, Family};
use crate::output::{GgaOutput, LdaOutput, MggaOutput};

/// Sealing module — keeps the [`EvaluateInput`] trait closed to the three
/// known input types (`LdaInput`, `GgaInput`, `MggaInput`). External crates
/// cannot add their own implementations because they cannot name
/// `sealed::Sealed`.
mod sealed {
    pub trait Sealed {}
}

/// Auto-dispatch trait used by [`crate::api::BatchEvaluator::evaluate`].
///
/// Each input type owns its family-specific dispatch call. Family
/// mismatch (e.g. passing an [`LdaInput`] to a Gga functional) returns
/// [`LibxcRsError::FamilyMismatch`] before any kernel is invoked.
///
/// # Errors
/// - [`LibxcRsError::FamilyMismatch`] — input's family does not match
///   the functional's `meta().family`.
/// - Any error returned by the underlying [`Functional::evaluate_lda`] /
///   [`Functional::evaluate_gga`] / [`Functional::evaluate_mgga`] call.
pub trait EvaluateInput: sealed::Sealed {
    /// Family-specific output bundle (`LdaOutput`, `GgaOutput`, or `MggaOutput`).
    type Output<'a>;

    /// The number of grid points this input represents. Used by
    /// [`crate::api::BatchEvaluator`] for the
    /// [`LibxcRsError::BatchOverflow`] guard.
    fn np(&self) -> usize;

    /// Dispatch this input to the matching `Functional::evaluate_*` method.
    fn dispatch(
        &self,
        functional: &Functional,
        order: DerivativeOrder,
        output: &mut Self::Output<'_>,
        workspace: &mut EvaluationWorkspace,
    ) -> Result<(), LibxcRsError>;
}

impl<'r> sealed::Sealed for LdaInput<'r> {}
impl<'r> EvaluateInput for LdaInput<'r> {
    type Output<'a> = LdaOutput<'a>;

    fn np(&self) -> usize {
        LdaInput::np(self)
    }

    fn dispatch(
        &self,
        functional: &Functional,
        order: DerivativeOrder,
        output: &mut Self::Output<'_>,
        workspace: &mut EvaluationWorkspace,
    ) -> Result<(), LibxcRsError> {
        // Convention: `expected` describes the functional's family,
        // `actual` describes the input's family. Mirrors
        // `LibxcRsError::FamilyMismatch`'s display: "functional {id}
        // is {expected:?}, but {actual:?} input provided".
        let func_family = functional.meta().family;
        if func_family != Family::Lda {
            return Err(LibxcRsError::FamilyMismatch {
                id: functional.meta().id,
                expected: func_family,
                actual: Family::Lda,
            });
        }
        functional.evaluate_lda(self, order, output, workspace)
    }
}

impl<'r> sealed::Sealed for GgaInput<'r> {}
impl<'r> EvaluateInput for GgaInput<'r> {
    type Output<'a> = GgaOutput<'a>;

    fn np(&self) -> usize {
        GgaInput::np(self)
    }

    fn dispatch(
        &self,
        functional: &Functional,
        order: DerivativeOrder,
        output: &mut Self::Output<'_>,
        workspace: &mut EvaluationWorkspace,
    ) -> Result<(), LibxcRsError> {
        let func_family = functional.meta().family;
        if func_family != Family::Gga {
            return Err(LibxcRsError::FamilyMismatch {
                id: functional.meta().id,
                expected: func_family,
                actual: Family::Gga,
            });
        }
        functional.evaluate_gga(self, order, output, workspace)
    }
}

impl<'r> sealed::Sealed for MggaInput<'r> {}
impl<'r> EvaluateInput for MggaInput<'r> {
    type Output<'a> = MggaOutput<'a>;

    fn np(&self) -> usize {
        MggaInput::np(self)
    }

    fn dispatch(
        &self,
        functional: &Functional,
        order: DerivativeOrder,
        output: &mut Self::Output<'_>,
        workspace: &mut EvaluationWorkspace,
    ) -> Result<(), LibxcRsError> {
        let func_family = functional.meta().family;
        if func_family != Family::Mgga {
            return Err(LibxcRsError::FamilyMismatch {
                id: functional.meta().id,
                expected: func_family,
                actual: Family::Mgga,
            });
        }
        functional.evaluate_mgga(self, order, output, workspace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::workspace::EvaluationWorkspace;
    use crate::input::LdaInput;
    use crate::model::{DerivativeOrder, FunctionalId, Spin};
    use crate::output::LdaOutput;

    /// Test 1: trivial proof that the three known impls compile together
    /// (the trait is sealed; an external crate cannot add its own impl).
    #[test]
    fn sealed_trait_compiles() {
        let rho = vec![0.1_f64, 0.2, 0.3, 0.4];
        let _lda = LdaInput::new(&rho, 4, Spin::Unpolarized).unwrap();
        let sigma = vec![0.0_f64; 4];
        let _gga = crate::input::GgaInput::new(&rho, &sigma, 4, Spin::Unpolarized).unwrap();
        let lapl = vec![0.0_f64; 4];
        let tau = vec![0.0_f64; 4];
        let _mgga = crate::input::MggaInput::new(&rho, &sigma, &lapl, &tau, 4, Spin::Unpolarized)
            .unwrap();
    }

    /// Test 2: BatchEvaluator-routed LDA evaluation must produce
    /// bit-identical output to a direct `Functional::evaluate_lda` call
    /// for a non-aux LDA functional (lda_x, id=1). Both paths run the
    /// same underlying dispatch function on the same workspace, so we
    /// assert byte-for-byte equality (not approximate).
    #[test]
    fn lda_dispatch_bit_equivalent() {
        let np = 4;
        let rho = vec![0.1_f64, 0.2, 0.3, 0.4];
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();

        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        // Path A: through BatchEvaluator
        let mut zk_a = vec![0.0_f64; np];
        {
            let mut be = crate::api::BatchEvaluator::new(Spin::Unpolarized, np);
            let mut out_a = LdaOutput::new(
                Some(&mut zk_a),
                None,
                None,
                None,
                None,
                np,
                Spin::Unpolarized,
            )
            .unwrap();
            be.evaluate(&f, &input, DerivativeOrder::Exc, &mut out_a)
                .unwrap();
        }

        // Path B: direct call into evaluate_lda
        let mut zk_b = vec![0.0_f64; np];
        {
            let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
            let mut out_b = LdaOutput::new(
                Some(&mut zk_b),
                None,
                None,
                None,
                None,
                np,
                Spin::Unpolarized,
            )
            .unwrap();
            f.evaluate_lda(&input, DerivativeOrder::Exc, &mut out_b, &mut ws)
                .unwrap();
        }

        for i in 0..np {
            assert_eq!(zk_a[i], zk_b[i], "zk[{i}] differs between BatchEvaluator and direct path");
        }
    }

    /// Test 3: passing an `LdaInput` to a GGA functional must return
    /// `LibxcRsError::FamilyMismatch` before any kernel runs.
    #[test]
    fn family_mismatch_lda_input_gga_func() {
        // gga_x_pbe is a GGA functional. Use canonical XC_-prefixed name.
        let id = FunctionalId::from_name("xc_gga_x_pbe")
            .unwrap_or_else(|_| FunctionalId::from_raw(101).unwrap());
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        assert_eq!(f.meta().family, Family::Gga);

        let np = 4;
        let rho = vec![0.1_f64, 0.2, 0.3, 0.4];
        let lda_input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        let mut zk = vec![0.0_f64; np];
        let mut be = crate::api::BatchEvaluator::new(Spin::Unpolarized, np);
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
        let err = be
            .evaluate(&f, &lda_input, DerivativeOrder::Exc, &mut out)
            .unwrap_err();
        match err {
            LibxcRsError::FamilyMismatch { expected, actual, .. } => {
                // Convention (mirrors error message): `expected` is the
                // functional's family, `actual` is the input's family.
                assert_eq!(expected, Family::Gga);
                assert_eq!(actual, Family::Lda);
            }
            other => panic!("expected FamilyMismatch, got {other:?}"),
        }
    }
}
