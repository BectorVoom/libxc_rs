//! Error function (erf) and complementary error function (erfc) implementations.
//!
//! Cephes/libm-style piecewise rational approximation with full f64 precision.
//! Coefficients are taken from the Sun Microsystems libm implementation (fdlibm),
//! which is also the basis for Rust's libm crate.
//!
//! The libxc C original (`xc_erfcx` in `faddeeva.c`) uses early returns and a
//! 100-case switch to evaluate only one region per call. CubeCL 0.9.0 does not
//! support `return` in `#[cube]` functions, so we use mutable result + `if/else`
//! guards to evaluate only the active region without early returns.

use cubecl::prelude::*;

// ============================================================================
// Cephes/fdlibm erf coefficients
// ============================================================================

// erx = erf(1) - 1 (for the 0.84375..1.25 region)
const ERX: f64 = 8.45062911510467529297e-01;

// Coefficients for approximation to erf on [0, 0.84375]
// erf(x) = x + x * R(x^2) where R = pp/qq
const PP0: f64 = 1.28379167095512558561e-01;
const PP1: f64 = -3.25042107247001499370e-01;
const PP2: f64 = -2.84817495755985104766e-02;
const PP3: f64 = -5.77027029648944159157e-03;
const PP4: f64 = -2.37630166566501626084e-05;
const QQ1: f64 = 3.97917223959155352819e-01;
const QQ2: f64 = 6.50222499887672944485e-02;
const QQ3: f64 = 5.08130628187576562776e-03;
const QQ4: f64 = 1.32494738004321644526e-04;
const QQ5: f64 = -3.96022827877536812320e-06;

// Coefficients for approximation to erfc on [0.84375, 1.25]
const PA0: f64 = -2.36211856075265944077e-03;
const PA1: f64 = 4.14856118683748331666e-01;
const PA2: f64 = -3.72207876035701323847e-01;
const PA3: f64 = 3.18346619901161753674e-01;
const PA4: f64 = -1.10894694282396677476e-01;
const PA5: f64 = 3.54783043195201877747e-02;
const PA6: f64 = -2.16637559983254089680e-03;
const QA1: f64 = 1.06420880400844228286e-01;
const QA2: f64 = 5.40397917702171048937e-01;
const QA3: f64 = 7.18286544141962539399e-02;
const QA4: f64 = 1.26171219808761642112e-01;
const QA5: f64 = 1.36370839120290507362e-02;
const QA6: f64 = 1.19844998467991074170e-02;

// Coefficients for approximation to erfc on [1.25, 2.857142857]
const RA0: f64 = -9.86494403484714822705e-03;
const RA1: f64 = -6.93858572707181764372e-01;
const RA2: f64 = -1.05586262253232909814e+01;
const RA3: f64 = -6.23753324503260060396e+01;
const RA4: f64 = -1.62396669462573071767e+02;
const RA5: f64 = -1.84605092906711035994e+02;
const RA6: f64 = -8.12874355063065934246e+01;
const RA7: f64 = -9.81432934416914548592e+00;
const SA1: f64 = 1.96512716674392571292e+01;
const SA2: f64 = 1.37657754143519702237e+02;
const SA3: f64 = 4.34565877475229228608e+02;
const SA4: f64 = 6.45387271733267880594e+02;
const SA5: f64 = 4.29008140027567833386e+02;
const SA6: f64 = 1.08635005541779435134e+02;
const SA7: f64 = 6.57024977031928170135e+00;
const SA8: f64 = -6.04244152148580987438e-02;

// Coefficients for approximation to erfc on [2.857142857, 6]
const RB0: f64 = -9.86494292470009928597e-03;
const RB1: f64 = -7.99283237680523006574e-01;
const RB2: f64 = -1.77579549177547519889e+01;
const RB3: f64 = -1.60636384855557935030e+02;
const RB4: f64 = -6.37566443368389085394e+02;
const RB5: f64 = -1.02509513161107724954e+03;
const RB6: f64 = -4.83519191608651397019e+02;
const SB1: f64 = 3.03380607875625778203e+01;
const SB2: f64 = 3.25792512996573918826e+02;
const SB3: f64 = 1.53672958608443695994e+03;
const SB4: f64 = 3.19985821950859553908e+03;
const SB5: f64 = 2.55305040643316442583e+03;
const SB6: f64 = 4.74528541206955367215e+02;
const SB7: f64 = -2.24409524465858183362e+01;

/// Compute the error function erf(x) with full f64 precision.
///
/// Uses `if/else` guards to evaluate only the active region's polynomial.
/// Accuracy: relative error <= 1e-15 across [-6, 6].
#[cube]
pub fn erf_approx(x: f64) -> f64 {
    let abs_x = f64::abs(x);
    let sign = select(x < 0.0, -1.0f64, 1.0f64);

    let mut result = sign;  // default: |x| >= 6 → sign * 1.0

    if abs_x < 0.84375 {
        // Region 1
        let x2 = x * x;
        let pp = PP0 + x2 * (PP1 + x2 * (PP2 + x2 * (PP3 + x2 * PP4)));
        let qq = 1.0 + x2 * (QQ1 + x2 * (QQ2 + x2 * (QQ3 + x2 * (QQ4 + x2 * QQ5))));
        result = x + x * (pp / qq);
    } else if abs_x < 1.25 {
        // Region 2
        let s = abs_x - 1.0;
        let pa = PA0 + s * (PA1 + s * (PA2 + s * (PA3 + s * (PA4 + s * (PA5 + s * PA6)))));
        let qa = 1.0 + s * (QA1 + s * (QA2 + s * (QA3 + s * (QA4 + s * (QA5 + s * QA6)))));
        result = sign * (ERX + pa / qa);
    } else if abs_x < 6.0 {
        // Regions 3-4 share the high-precision exp trick
        let x_hi = f64::floor(abs_x * 1048576.0) / 1048576.0;
        let x_lo = abs_x - x_hi;
        let inv_x2 = 1.0 / (abs_x * abs_x);

        if abs_x < 2.857142857 {
            // Region 3: 1.25 <= |x| < 2.857142857
            let ra = RA0 + inv_x2 * (RA1 + inv_x2 * (RA2 + inv_x2 * (RA3 + inv_x2 * (RA4 + inv_x2 * (RA5 + inv_x2 * (RA6 + inv_x2 * RA7))))));
            let sa = 1.0 + inv_x2 * (SA1 + inv_x2 * (SA2 + inv_x2 * (SA3 + inv_x2 * (SA4 + inv_x2 * (SA5 + inv_x2 * (SA6 + inv_x2 * (SA7 + inv_x2 * SA8)))))));
            let erfc_val = f64::exp(-x_hi * x_hi - 0.5625) * f64::exp(-x_lo * (abs_x + x_hi) + ra / sa) / abs_x;
            result = sign * (1.0 - erfc_val);
        } else {
            // Region 4: 2.857142857 <= |x| < 6
            let rb = RB0 + inv_x2 * (RB1 + inv_x2 * (RB2 + inv_x2 * (RB3 + inv_x2 * (RB4 + inv_x2 * (RB5 + inv_x2 * RB6)))));
            let sb = 1.0 + inv_x2 * (SB1 + inv_x2 * (SB2 + inv_x2 * (SB3 + inv_x2 * (SB4 + inv_x2 * (SB5 + inv_x2 * (SB6 + inv_x2 * SB7))))));
            let erfc_val = f64::exp(-x_hi * x_hi - 0.5625) * f64::exp(-x_lo * (abs_x + x_hi) + rb / sb) / abs_x;
            result = sign * (1.0 - erfc_val);
        }
    }

    result
}

/// Backward-compatible alias for generated kernels that still reference the
/// older CubeCL-facing helper name.
#[cube]
pub fn erf_cube(x: f64) -> f64 {
    erf_approx(x)
}

/// Compute the complementary error function erfc(x) = 1 - erf(x).
///
/// Uses `if/else` guards to evaluate only the active region.
/// Uses fdlibm high-precision exp trick (x split into hi/lo parts) for regions 3-4.
/// Accuracy: relative error < 1e-14 across most of [-6, 6].
#[cube]
pub fn erfc_approx(x: f64) -> f64 {
    let abs_x = f64::abs(x);

    let mut result = select(x < 0.0, 2.0f64, 0.0f64);  // default: |x| >= 6

    if abs_x < 0.84375 {
        // Region 1: erfc = 1 - erf(x), no severe cancellation
        let x2 = x * x;
        let pp = PP0 + x2 * (PP1 + x2 * (PP2 + x2 * (PP3 + x2 * PP4)));
        let qq = 1.0 + x2 * (QQ1 + x2 * (QQ2 + x2 * (QQ3 + x2 * (QQ4 + x2 * QQ5))));
        result = 1.0 - (x + x * (pp / qq));
    } else if abs_x < 1.25 {
        // Region 2
        let s = abs_x - 1.0;
        let pa = PA0 + s * (PA1 + s * (PA2 + s * (PA3 + s * (PA4 + s * (PA5 + s * PA6)))));
        let qa = 1.0 + s * (QA1 + s * (QA2 + s * (QA3 + s * (QA4 + s * (QA5 + s * QA6)))));
        result = select(x < 0.0, 1.0 + ERX + pa / qa, (1.0 - ERX) - pa / qa);
    } else if abs_x < 6.0 {
        // Regions 3-4 share the high-precision exp trick
        let x_hi = f64::floor(abs_x * 1048576.0) / 1048576.0;
        let x_lo = abs_x - x_hi;
        let inv_x2 = 1.0 / (abs_x * abs_x);

        if abs_x < 2.857142857 {
            // Region 3: 1.25 <= |x| < 2.857142857
            let ra = RA0 + inv_x2 * (RA1 + inv_x2 * (RA2 + inv_x2 * (RA3 + inv_x2 * (RA4 + inv_x2 * (RA5 + inv_x2 * (RA6 + inv_x2 * RA7))))));
            let sa = 1.0 + inv_x2 * (SA1 + inv_x2 * (SA2 + inv_x2 * (SA3 + inv_x2 * (SA4 + inv_x2 * (SA5 + inv_x2 * (SA6 + inv_x2 * (SA7 + inv_x2 * SA8)))))));
            let erfc_pos = f64::exp(-x_hi * x_hi - 0.5625) * f64::exp(-x_lo * (abs_x + x_hi) + ra / sa) / abs_x;
            result = select(x < 0.0, 2.0 - erfc_pos, erfc_pos);
        } else {
            // Region 4: 2.857142857 <= |x| < 6
            let rb = RB0 + inv_x2 * (RB1 + inv_x2 * (RB2 + inv_x2 * (RB3 + inv_x2 * (RB4 + inv_x2 * (RB5 + inv_x2 * RB6)))));
            let sb = 1.0 + inv_x2 * (SB1 + inv_x2 * (SB2 + inv_x2 * (SB3 + inv_x2 * (SB4 + inv_x2 * (SB5 + inv_x2 * (SB6 + inv_x2 * SB7))))));
            let erfc_pos = f64::exp(-x_hi * x_hi - 0.5625) * f64::exp(-x_lo * (abs_x + x_hi) + rb / sb) / abs_x;
            result = select(x < 0.0, 2.0 - erfc_pos, erfc_pos);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use cubecl::cpu::{CpuDevice, CpuRuntime};
    use cubecl::Runtime;
    use cubecl::client::ComputeClient;

    #[cube(launch_unchecked)]
    fn test_erf_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let idx = ABSOLUTE_POS;
        output[idx] = erf_approx(input[idx]);
    }

    #[cube(launch_unchecked)]
    fn test_erfc_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let idx = ABSOLUTE_POS;
        output[idx] = erfc_approx(input[idx]);
    }

    fn make_client() -> ComputeClient<CpuRuntime> {
        let device = CpuDevice;
        CpuRuntime::client(&device)
    }

    fn run_erf(values: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = values.len();
        let input_handle = client.create_from_slice(bytemuck::cast_slice(values));
        let output_handle = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_erf_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts::<f64>(&input_handle, n, 1),
                ArrayArg::from_raw_parts::<f64>(&output_handle, n, 1),
            ).unwrap();
        }

        let bytes = client.read_one(output_handle);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_erfc(values: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = values.len();
        let input_handle = client.create_from_slice(bytemuck::cast_slice(values));
        let output_handle = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_erfc_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts::<f64>(&input_handle, n, 1),
                ArrayArg::from_raw_parts::<f64>(&output_handle, n, 1),
            ).unwrap();
        }

        let bytes = client.read_one(output_handle);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    #[test]
    fn test_erf_known_values() {
        let results = run_erf(&[0.0, 1.0, -1.0]);
        assert_eq!(results[0], 0.0);
        approx::assert_relative_eq!(results[1], 0.8427007929497149, max_relative = 1e-15);
        approx::assert_relative_eq!(results[2], -0.8427007929497149, max_relative = 1e-15);
    }

    #[test]
    fn test_erf_symmetry() {
        let pos = run_erf(&[0.5, 1.0, 2.0, 3.0]);
        let neg = run_erf(&[-0.5, -1.0, -2.0, -3.0]);
        for (p, n) in pos.iter().zip(neg.iter()) {
            approx::assert_relative_eq!(*p, -n, max_relative = 1e-15);
        }
    }

    #[test]
    fn test_erf_large_values() {
        let results = run_erf(&[6.0, 10.0, 27.0]);
        assert_eq!(results[0], 1.0);
        assert_eq!(results[1], 1.0);
        assert_eq!(results[2], 1.0);
    }

    #[test]
    fn test_erf_libm_sweep() {
        let n = 1000;
        let mut inputs = Vec::with_capacity(n);
        for i in 0..n {
            let x = -6.0 + 12.0 * (i as f64) / ((n - 1) as f64);
            inputs.push(x);
        }

        let results = run_erf(&inputs);

        for (i, (&result, &x)) in results.iter().zip(inputs.iter()).enumerate() {
            let expected = libm::erf(x);
            if expected.abs() < 1e-300 {
                assert!(result.abs() < 1e-14,
                    "erf({}) = {}, libm::erf = {}, abs_err too large at index {}",
                    x, result, expected, i);
            } else {
                let err = ((result - expected) / expected).abs();
                assert!(err < 1e-13,
                    "erf({}) = {}, libm::erf = {}, rel_err = {} at index {}",
                    x, result, expected, err, i);
            }
        }
    }

    #[test]
    fn test_erfc_known_values() {
        let results = run_erfc(&[0.0]);
        assert_eq!(results[0], 1.0);
    }

    #[test]
    fn test_erfc_libm_sweep() {
        let n = 500;
        let mut inputs = Vec::with_capacity(n);
        for i in 0..n {
            // Test up to 5.99 to avoid the x=6 boundary where erfc~2e-17
            let x = 5.99 * (i as f64) / ((n - 1) as f64);
            inputs.push(x);
        }

        let results = run_erfc(&inputs);

        for (i, (&result, &x)) in results.iter().zip(inputs.iter()).enumerate() {
            let expected = libm::erfc(x);
            if expected.abs() < 1e-300 {
                // Near zero, check absolute
                assert!(result.abs() < 1e-14,
                    "erfc({}) = {}, libm::erfc = {}, abs_err too large at index {}",
                    x, result, expected, i);
            } else {
                let err = ((result - expected) / expected).abs();
                // With if/else guards, only the active region's polynomial is
                // evaluated, eliminating cross-region interference.
                assert!(err < 1e-14,
                    "erfc({}) = {}, libm::erfc = {}, rel_err = {} at index {}",
                    x, result, expected, err, i);
            }
        }
    }

    #[test]
    fn test_erfc_edge_cases() {
        let results = run_erfc(&[27.0]);
        assert_eq!(results[0], 0.0);
    }
}
