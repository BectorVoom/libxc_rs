//! `Functional::evaluate_{lda,gga,mgga}` — top-level evaluation entry points.
//!
//! Routes per-family evaluation through either the direct dispatch table
//! (`auxiliaries.is_empty()`) or the mixed-evaluation accumulator
//! (`evaluate_mixed_{lda,gga,mgga}_functional`). This is the public API
//! surface most users will interact with for hybrid functionals.
//!
//! Pitfall 7 (Plan 05-03): Functionals deferred at the kernel layer
//! (e.g. `LDA_C_PK09`, `MGGA_X_BR89`) succeed at `Functional::new` (so
//! metadata queries and aux iteration still work) but return
//! `UnsupportedFunctional` here at evaluate time via the inner
//! `from_id` helpers.

use crate::error::LibxcRsError;
use crate::eval::dispatch::dispatch_lda;
use crate::eval::gga_dispatch::dispatch_gga;
use crate::eval::mgga_dispatch::dispatch_mgga;
use crate::eval::mix::{evaluate_mixed_gga, evaluate_mixed_lda_functional, evaluate_mixed_mgga};
use crate::eval::workspace::EvaluationWorkspace;
use crate::functional::Functional;
use crate::input::{GgaInput, LdaInput, MggaInput};
use crate::model::{DerivativeOrder, GgaFunctional, LdaFunctional, MggaFunctional};
use crate::output::{GgaOutput, LdaOutput, MggaOutput};

impl Functional {
    /// Evaluate this LDA functional over `input` at the requested derivative
    /// order, writing results into `output`.
    ///
    /// Routes to direct `dispatch_lda` when this functional has no
    /// auxiliaries (semilocal/non-hybrid), or to the mixed-evaluation
    /// accumulator when it does. The `workspace` argument is only used
    /// in the mixed path; it can be a freshly-allocated workspace or a
    /// reused one (zeroed before each aux per-aux as needed).
    pub fn evaluate_lda(
        &self,
        input: &LdaInput,
        order: DerivativeOrder,
        output: &mut LdaOutput,
        workspace: &mut EvaluationWorkspace,
    ) -> Result<(), LibxcRsError> {
        if self.auxiliaries.is_empty() {
            let lda_fn = LdaFunctional::from_id(self.meta.id)?;
            dispatch_lda(
                lda_fn,
                input,
                order,
                output,
                &*self.params,
                &self.thresholds,
            )
        } else {
            evaluate_mixed_lda_functional(self, input, order, output, workspace)
        }
    }

    /// Evaluate this GGA functional. See [`Functional::evaluate_lda`] for
    /// routing semantics.
    pub fn evaluate_gga(
        &self,
        input: &GgaInput,
        order: DerivativeOrder,
        output: &mut GgaOutput,
        workspace: &mut EvaluationWorkspace,
    ) -> Result<(), LibxcRsError> {
        if self.auxiliaries.is_empty() {
            let gga_fn = GgaFunctional::from_id(self.meta.id)?;
            dispatch_gga(
                gga_fn,
                input,
                order,
                output,
                &*self.params,
                &self.thresholds,
            )
        } else {
            evaluate_mixed_gga(self, input, order, output, workspace)
        }
    }

    /// Evaluate this MGGA functional. See [`Functional::evaluate_lda`] for
    /// routing semantics.
    pub fn evaluate_mgga(
        &self,
        input: &MggaInput,
        order: DerivativeOrder,
        output: &mut MggaOutput,
        workspace: &mut EvaluationWorkspace,
    ) -> Result<(), LibxcRsError> {
        if self.auxiliaries.is_empty() {
            let mgga_fn = MggaFunctional::from_id(self.meta.id)?;
            dispatch_mgga(
                mgga_fn,
                input,
                order,
                output,
                &*self.params,
                &self.thresholds,
            )
        } else {
            evaluate_mixed_mgga(self, input, order, output, workspace)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::workspace::EvaluationWorkspace;
    use crate::eval::{dispatch_lda, LdaFunctionalParams};
    use crate::input::LdaInput;
    use crate::model::{FunctionalId, Spin, Thresholds};
    use crate::output::LdaOutput;

    /// Test 5: For a non-aux LDA functional, `Functional::evaluate_lda`
    /// must produce bit-identical output to a direct `dispatch_lda` call
    /// with the same params/thresholds.
    #[test]
    fn evaluate_lda_no_aux_matches_direct_dispatch() {
        let np = 4;
        let rho = vec![0.1, 0.2, 0.5, 1.0];
        let id = FunctionalId::from_raw(1).unwrap(); // lda_x
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        assert!(f.auxiliaries.is_empty(), "lda_x must have no aux on current metadata");

        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        // Functional path
        let mut zk_f = vec![0.0_f64; np];
        let mut vrho_f = vec![0.0_f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out = LdaOutput::new(
                Some(&mut zk_f),
                Some(&mut vrho_f),
                None,
                None,
                None,
                np,
                Spin::Unpolarized,
            )
            .unwrap();
            f.evaluate_lda(&input, DerivativeOrder::Vxc, &mut out, &mut ws).unwrap();
        }

        // Direct dispatch path
        let mut zk_d = vec![0.0_f64; np];
        let mut vrho_d = vec![0.0_f64; np];
        {
            let mut out = LdaOutput::new(
                Some(&mut zk_d),
                Some(&mut vrho_d),
                None,
                None,
                None,
                np,
                Spin::Unpolarized,
            )
            .unwrap();
            dispatch_lda(
                LdaFunctional::LdaX,
                &input,
                DerivativeOrder::Vxc,
                &mut out,
                &LdaFunctionalParams::default(),
                &Thresholds::default(),
            )
            .unwrap();
        }

        for i in 0..np {
            assert_eq!(zk_f[i], zk_d[i], "zk[{i}] differs");
            assert_eq!(vrho_f[i], vrho_d[i], "vrho[{i}] differs");
        }
    }

    /// Pitfall 7 / Test 9: a deferred LDA id constructs successfully (so
    /// metadata queries work) but `evaluate_lda` returns
    /// `UnsupportedFunctional`.
    #[test]
    fn evaluate_lda_deferred_id_returns_unsupported() {
        // lda_c_pk09 = 554 is a deferred LDA id (see model::lda_functional).
        let id = FunctionalId::from_raw(554).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        // Construction succeeds, metadata is queryable.
        assert_eq!(f.meta.id.raw(), 554);

        let np = 2;
        let rho = vec![0.1_f64, 0.5];
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0_f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        let mut out = LdaOutput::new(Some(&mut zk), None, None, None, None, np, Spin::Unpolarized)
            .unwrap();
        let err = f.evaluate_lda(&input, DerivativeOrder::Exc, &mut out, &mut ws).unwrap_err();
        match err {
            LibxcRsError::UnsupportedFunctional { id: e_id, .. } => {
                assert_eq!(e_id.raw(), 554);
            }
            other => panic!("expected UnsupportedFunctional, got {other:?}"),
        }
    }
}
