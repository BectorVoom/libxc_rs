//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 822/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk822<F: Float>(t7841: F, t865: F, t2718: F, t25049: F, t4234: F, t7101: F, t1510: F, t24269: F, t1499: F, t2051: F, t23003: F, t23026: F, t23029: F, t23167: F, t23170: F, t24246: F, t24250: F, t24265: F, t25239: F, t25243: F, t25246: F, t25252: F, t25259: F, t2617: F, t4162: F, t4166: F, t7102: F, t7104: F, t7837: F, t812: F) -> (F, F, F, F) {
    let t26581 = t7841 * t865;
    let t26582 = t2718 * t26581;
    let t26591 = F::new(0.38381794893125283518e-1) * t25049;
    let t26598 = t7101 * t4234;
    let t26608 = t24269 * t1510;
    let t26611 = -F::new(0.16449340668482264365e-1) * t25239 - t812 * t26598 - F::new(0.16449340668482264365e-1) * t25243 + F::new(0.82246703342411321825e-2) * t25246 + F::new(0.9869604401089358619e-1) * t25252 + t23003 - F::new(0.82246703342411321825e-2) * t25259 + t24246 + t1499 * t7104 - F::new(0.82246703342411321825e-2) * t23026 - t23029 + t24250 - t4166 * t7102 - t2617 * t7837 - t812 * t26608 + t4162 * t2051 + t23167 + t23170 - t24265;
    (t26581, t26582, t26591, t26611)
}
