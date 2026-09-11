//! `Functional::evaluate_{lda,gga,mgga}` — top-level evaluation entry points.
//!
//! Routes per-family evaluation through either the direct dispatch table
//! (`auxiliaries.is_empty()`) or the mixed-evaluation accumulator
//! (`evaluate_mixed_{lda,gga,mgga}_functional`). This is the public API
//! surface most users will interact with for hybrid functionals.
//!
//! A functional with no route to a kernel still constructs (so metadata
//! queries and aux iteration work) and returns `UnsupportedFunctional` here at
//! evaluate time. `verify/tests/refusal_sweep.rs` lists every public id that
//! does.

use libxc_core::error::LibxcRsError;
// 11-12 (G-2): import the eval-level dispatch_* (real under the family feature,
// stub when off) so this router needs no per-family cfg.
use crate::eval::{dispatch_gga_by_id, dispatch_lda_by_id, dispatch_mgga_by_id};
use crate::eval::mix::{
    add_own_kernel_mgga, evaluate_mixed_gga, evaluate_mixed_lda_functional,
    evaluate_mixed_mgga,
};
use crate::eval::workspace::EvaluationWorkspace;
use crate::functional::Functional;
use libxc_core::input::{GgaInput, LdaInput, MggaInput};
use libxc_core::model::DerivativeOrder;
use libxc_core::output::{GgaOutput, LdaOutput, MggaOutput};

impl Functional {
    /// Evaluate this LDA functional over `input` at the requested derivative
    /// order, writing results into `output`.
    ///
    /// Routes to direct `dispatch_lda_by_id` when this functional has no
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
            dispatch_lda_by_id(
                self.meta.id,
                input,
                order,
                output,
                self.kernel_ext_params(),
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
            dispatch_gga_by_id(
                self.meta.id,
                input,
                order,
                output,
                self.kernel_ext_params(),
                &self.thresholds,
            )
        } else {
            if let Some(r) = self.try_fused_gga(input, order, output, workspace) {
                return r;
            }
            evaluate_mixed_gga(self, input, order, output, workspace)
        }
    }

    /// The fused path for a composite: one generated kernel evaluating every
    /// auxiliary in the same loop (`libxc_reval::fused`, emitted by
    /// `tools/translate_rayon/fuse.py`), bit-identical to the mix and with no
    /// scratch at all. `None` when it does not apply -- the caller then runs
    /// the mix, which is what every composite without a fused kernel does.
    ///
    /// The workspace is not used on this path, but a mismatched one is still
    /// an error on the mix path, and the two must agree on what they reject.
    fn try_fused_gga(
        &self,
        input: &GgaInput,
        order: DerivativeOrder,
        output: &mut GgaOutput,
        workspace: &EvaluationWorkspace,
    ) -> Option<Result<(), LibxcRsError>> {
        use libxc_reval::fused::{FusedLeg, try_fused_gga};

        /// A fused kernel has a fixed, small number of legs; anything larger
        /// is not one and is not worth building a leg list for.
        const MAX_LEGS: usize = 4;

        if !crate::eval::fused_enabled()
            || self.auxiliaries.len() > MAX_LEGS
            || workspace.np() != input.np()
            || workspace.spin() != input.spin()
        {
            return None;
        }
        let mut legs = [FusedLeg { id: 0, ext: None, thresholds: self.thresholds }; MAX_LEGS];
        for (slot, aux) in legs.iter_mut().zip(self.auxiliaries.iter()) {
            *slot = FusedLeg {
                id: aux.meta.id.raw(),
                ext: aux.kernel_ext_params(),
                thresholds: aux.thresholds,
            };
        }
        try_fused_gga(
            self.meta.id.raw(),
            &legs[..self.auxiliaries.len()],
            &self.mix_coefficients,
            input,
            output,
            order,
            input.spin(),
        )
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
        // A deorbitalized meta-GGA (SCAN-L and relatives) is neither a kernel
        // nor a mix: see `crate::eval::deorbitalize`.
        if libxc_core::meta::deorbitalized(self.meta.id).is_some() {
            return crate::eval::deorbitalize::evaluate(self, input, order, output);
        }
        if self.auxiliaries.is_empty() {
            dispatch_mgga_by_id(
                self.meta.id,
                input,
                order,
                output,
                self.kernel_ext_params(),
                &self.thresholds,
            )
        } else {
            // The mix first: it zeroes the caller's buffers and accumulates
            // every auxiliary into them.
            evaluate_mixed_mgga(self, input, order, output, workspace)?;
            // Then the functional's own kernel on top, if it has one. libxc
            // runs both with no guard between them; see `add_own_kernel_mgga`.
            // `hyb_mgga_xc_b0kcis` is the only functional this applies to.
            if libxc_reval::routing::mgga_has_own_kernel(self.meta.id) {
                add_own_kernel_mgga(self, input, order, output, workspace)?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::workspace::EvaluationWorkspace;
    use crate::eval::{dispatch_lda, LdaFunctionalParams};
    use libxc_core::input::LdaInput;
    use libxc_core::model::{FunctionalId, LdaFunctional, Spin, Thresholds};
    use libxc_core::output::LdaOutput;

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

    /// `lda_c_pk09` (554) sat on the CubeCL-era deferred list, so every order
    /// refused. libxc ships it at exc through kxc and has no lxc; it now
    /// evaluates the orders it has and refuses the one it does not, as libxc
    /// does.
    #[test]
    fn lda_c_pk09_evaluates_its_claimed_orders_only() {
        let id = FunctionalId::from_raw(554).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();

        let np = 2;
        let rho = vec![0.1_f64, 0.5];
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        let mut zk = vec![0.0_f64; np];
        let mut out = LdaOutput::new(Some(&mut zk), None, None, None, None, np, Spin::Unpolarized)
            .unwrap();
        f.evaluate_lda(&input, DerivativeOrder::Exc, &mut out, &mut ws).unwrap();
        assert!(zk.iter().all(|v| v.is_finite() && *v != 0.0), "zk = {zk:?}");

        let mut b = [vec![0.0_f64; np], vec![0.0; np], vec![0.0; np], vec![0.0; np], vec![0.0; np]];
        let [z, v1, v2, v3, v4] = &mut b;
        let mut out = LdaOutput::new(Some(z), Some(v1), Some(v2), Some(v3), Some(v4), np, Spin::Unpolarized)
            .unwrap();
        match f.evaluate_lda(&input, DerivativeOrder::Lxc, &mut out, &mut ws).unwrap_err() {
            LibxcRsError::UnsupportedDerivativeOrder { order, max, .. } => {
                assert_eq!(order, DerivativeOrder::Lxc);
                assert_eq!(max, DerivativeOrder::Kxc);
            }
            other => panic!("expected UnsupportedDerivativeOrder, got {other:?}"),
        }
    }
}
