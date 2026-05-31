//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1400/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1400<F: Float>(t23518: F, t23634: F, t1003: F, t1022: F, t10359: F, t1058: F, t1060: F, t11037: F, t1953: F, t23571: F, t23604: F, t23609: F, t23633: F, t23635: F, t23657: F, t23658: F, t23662: F, t23670: F, t23678: F, t23707: F, t3076: F, t3120: F, t353: F, t383: F, t43240: F, t607: F, t6797: F, t6800: F, t6813: F, t83226: F, t83233: F, t83234: F, t83239: F, t83240: F, t83245: F, t83246: F, t83247: F) -> F {
    let t83265 = t23518 * t23634;
    let t83270 = -F::cast_from(3.0_f64) * t11037 * t23662 + F::cast_from(3.0_f64) * t1058 * t23571 * t1022 * t1060 + F::cast_from(3.0_f64) * t1003 * t23707 + t353 * t383 * t83226 + F::cast_from(0.82246703342411321826e-2_f64) * t23633 * t23635 * t43240 * t6800 - F::cast_from(0.16449340668482264365e-1_f64) * t23633 * t83233 * t83234 + F::cast_from(0.10966227112321509577e-1_f64) * t83239 * t83240 * t83234 + F::cast_from(0.16449340668482264365e-1_f64) * t83245 * t83246 * t83247 * t23678 + F::cast_from(0.82246703342411321826e-2_f64) * t23633 * t23635 * t607 * t3120 * t6800 + t10359 * t1953 + F::cast_from(3.0_f64) * t3076 * t6813 + F::cast_from(0.13159472534785811492e0_f64) * t23670 * t23658 - F::cast_from(0.49348022005446793095e-1_f64) * t6797 * t23657 * t23609 - F::cast_from(0.82246703342411321826e-2_f64) * t83245 * t83265 * t83247 * t23604;
    t83270
}
