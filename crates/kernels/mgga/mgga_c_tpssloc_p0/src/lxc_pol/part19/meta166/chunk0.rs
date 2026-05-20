//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 793/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk793<F: Float>(t2710: F, t798: F, t116: F, t229: F, t212: F, t776: F, t2586: F, t210: F, t214: F, t9516: F, t597: F, t60: F) -> (F, F, F, F, F, F, F) {
    let t9520 = t798 * t2710;
    let t9523 = t229 * t116;
    let t9524 = t212 * t776;
    let t9525 = t9523 * t9524;
    let t9526 = t2586 * t9525;
    let t9529 = t210 * t214 * t9516;
    let t9533 = F::new(1.0) / t60 / t597;
    (t9520, t9523, t9524, t9525, t9526, t9529, t9533)
}
