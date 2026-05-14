//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 767/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk767<F: Float>(t109: F, t27005: F, t27065: F, t27127: F, t27141: F, t533: F, t1390: F, t671: F, t7890: F, t2075: F, t4072: F, t2039: F, t5107: F, t26127: F, t22472: F, t23912: F, t26130: F, t26132: F) -> (F, F, F, F, F, F) {
    let t110 = 1.0 < t109;
    let t27143 = t27005 + t27065 + t27127 + t27141;
    let t27144 = t533 * t27143;
    let t27145 = t27144 * t1390;
    let t27147 = t7890 * t671;
    let t27150 = t2075 * t4072;
    let t27163 = t5107 * t2039;
    let t27166 = 2.0 / 3.0 * t26127;
    let t27170 = piecewise3(t110, 0.0, t23912 + t22472 + t27166 + t26130 / 2.0 - t26132 / 4.0);
    (t27143, t27145, t27147, t27150, t27163, t27170)
}
