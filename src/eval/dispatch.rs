//! Match-based dispatch layer for LDA kernel evaluation.
//!
//! Routes evaluation requests to the correct CubeCL kernel function based on
//! derivative order and spin mode. All kernel launches go through the safe
//! wrappers in `kernel::lda::launch_lda_x` -- this module contains NO unsafe code
//! (BUILD-04 compliant).

use crate::error::LibxcRsError;
use crate::input::LdaInput;
use crate::model::{DerivativeOrder, Thresholds};
use crate::output::LdaOutput;

/// Evaluate an LDA functional on the given input, writing results to output.
///
/// Routes to the correct kernel based on derivative order and spin mode.
/// Zeros caller output buffers before evaluation. Handles `None` output
/// fields by allocating dummy buffers the kernel writes to but whose
/// results are discarded.
///
/// # Arguments
/// * `input` - Validated LDA input bundle
/// * `order` - Maximum derivative order to compute
/// * `output` - Output bundle with optional buffers for each derivative level
/// * `alpha` - Functional mixing parameter (1.0 for pure LDA_X)
/// * `thresholds` - Numerical thresholds for evaluation stability
///
/// # Errors
/// Returns `LibxcRsError` if evaluation fails.
pub fn dispatch_lda(
    _input: &LdaInput,
    _order: DerivativeOrder,
    _output: &mut LdaOutput,
    _alpha: f64,
    _thresholds: &Thresholds,
) -> Result<(), LibxcRsError> {
    todo!("dispatch_lda not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::LdaInput;
    use crate::model::{DerivativeOrder, Spin, Thresholds};
    use crate::output::LdaOutput;

    fn default_thresholds() -> Thresholds {
        Thresholds::default()
    }

    #[test]
    fn test_exc_unpolarized_produces_negative_energy() {
        let rho = vec![0.1, 0.2, 0.5, 1.0];
        let np = 4;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Exc, &mut output, 1.0, &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        for (i, &val) in zk_result.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
        }
    }

    #[test]
    fn test_vxc_unpolarized_populates_both_zk_and_vrho() {
        let rho = vec![0.1, 0.5];
        let np = 2;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Vxc, &mut output, 1.0, &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        for (i, &val) in zk_result.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
        }
        let vrho_result = output.vrho.unwrap();
        for (i, &val) in vrho_result.iter().enumerate() {
            assert!(val != 0.0, "vrho[{i}] should be non-zero");
        }
    }

    #[test]
    fn test_vxc_with_vrho_none_still_succeeds() {
        let rho = vec![0.1, 0.5];
        let np = 2;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        // vrho is None but order is Vxc -- should succeed with dummy buffer
        dispatch_lda(&input, DerivativeOrder::Vxc, &mut output, 1.0, &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        for (i, &val) in zk_result.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative");
        }
    }

    #[test]
    fn test_exc_polarized_routes_to_pol_kernel() {
        let np = 2;
        // Polarized: 2 components per point
        let rho = vec![0.1, 0.05, 0.2, 0.1];
        let input = LdaInput::new(&rho, np, Spin::Polarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Polarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Exc, &mut output, 1.0, &default_thresholds()).unwrap();

        let zk_result = output.zk.unwrap();
        for (i, &val) in zk_result.iter().enumerate() {
            assert!(val < 0.0, "zk[{i}] = {val} should be negative (polarized)");
        }
    }

    #[test]
    fn test_vxc_polarized_vrho_has_2np_elements() {
        let np = 2;
        let rho = vec![0.1, 0.05, 0.2, 0.1];
        let input = LdaInput::new(&rho, np, Spin::Polarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np * 2]; // Polarized: 2 components per point
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), None, None, None, np, Spin::Polarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Vxc, &mut output, 1.0, &default_thresholds()).unwrap();

        let vrho_result = output.vrho.unwrap();
        assert_eq!(vrho_result.len(), np * 2);
        for (i, &val) in vrho_result.iter().enumerate() {
            assert!(val != 0.0, "vrho[{i}] should be non-zero (polarized)");
        }
    }

    #[test]
    fn test_dispatch_zeros_output_buffers() {
        let rho = vec![0.1];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![999.0f64; np]; // Pre-filled with non-zero
        let mut output = LdaOutput::new(
            Some(&mut zk), None, None, None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Exc, &mut output, 1.0, &default_thresholds()).unwrap();

        // The result should NOT be 999.0 + kernel_value; it should be just kernel_value
        // because dispatch zeros the buffer before launch
        let zk_result = output.zk.unwrap();
        assert!(zk_result[0] < 0.0, "zk should be negative, not contaminated by pre-fill");
        // Exchange energy for rho=0.1 is approximately -0.34 (LDA_X Slater exchange)
        assert!(zk_result[0] > -2.0, "zk = {} seems too negative (pre-fill contamination?)", zk_result[0]);
    }

    #[test]
    fn test_fxc_unpolarized_populates_all_levels() {
        let rho = vec![0.5];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np];
        let mut v2rho2 = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), Some(&mut v2rho2), None, None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Fxc, &mut output, 1.0, &default_thresholds()).unwrap();

        assert!(output.zk.unwrap()[0] < 0.0, "zk should be negative");
        assert!(output.vrho.unwrap()[0] != 0.0, "vrho should be non-zero");
        assert!(output.v2rho2.unwrap()[0] != 0.0, "v2rho2 should be non-zero");
    }

    #[test]
    fn test_kxc_unpolarized_populates_all_levels() {
        let rho = vec![0.5];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np];
        let mut v2rho2 = vec![0.0f64; np];
        let mut v3rho3 = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), Some(&mut v2rho2), Some(&mut v3rho3), None, np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Kxc, &mut output, 1.0, &default_thresholds()).unwrap();

        assert!(output.zk.unwrap()[0] < 0.0, "zk should be negative");
        assert!(output.vrho.unwrap()[0] != 0.0, "vrho should be non-zero");
        assert!(output.v2rho2.unwrap()[0] != 0.0, "v2rho2 should be non-zero");
        assert!(output.v3rho3.unwrap()[0] != 0.0, "v3rho3 should be non-zero");
    }

    #[test]
    fn test_lxc_unpolarized_populates_all_levels() {
        let rho = vec![0.5];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np];
        let mut v2rho2 = vec![0.0f64; np];
        let mut v3rho3 = vec![0.0f64; np];
        let mut v4rho4 = vec![0.0f64; np];
        let mut output = LdaOutput::new(
            Some(&mut zk), Some(&mut vrho), Some(&mut v2rho2), Some(&mut v3rho3), Some(&mut v4rho4), np, Spin::Unpolarized,
        ).unwrap();

        dispatch_lda(&input, DerivativeOrder::Lxc, &mut output, 1.0, &default_thresholds()).unwrap();

        assert!(output.zk.unwrap()[0] < 0.0, "zk should be negative");
        assert!(output.vrho.unwrap()[0] != 0.0, "vrho should be non-zero");
        assert!(output.v2rho2.unwrap()[0] != 0.0, "v2rho2 should be non-zero");
        assert!(output.v3rho3.unwrap()[0] != 0.0, "v3rho3 should be non-zero");
        assert!(output.v4rho4.unwrap()[0] != 0.0, "v4rho4 should be non-zero");
    }
}
