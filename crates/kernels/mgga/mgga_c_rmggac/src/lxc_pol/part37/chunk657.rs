//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 657/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk657<F: Float>(t14559: F, t2160: F, t638: F, t70237: F, t14580: F, t899: F, t70328: F, t70376: F, t70385: F, t70439: F, t2228: F, t265: F, t739: F, t69108: F, t69114: F, t14512: F, t7269: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t71720 = t638 * t2160 * t14559;
    let t71744 = 0.60975299583150056624e-3 * t70237;
    let t71772 = t899 * t14580;
    let t71789 = 0.3830813990396805546e-3 * t70328;
    let t71802 = 0.162600798888400151e-2 * t70376;
    let t71804 = 0.32526727992809621482e-4 * t70385;
    let t71832 = 0.2316441583394736328e-4 * t70439;
    let t71835 = t2228 * t265;
    let t71836 = t739 * t71835;
    let t71852 = 0.10492326631435615411e0 * t69108;
    let t71854 = 0.66671395154821946452e-1 * t69114;
    let t71863 = t14512 * t7269;
    (t71720, t71744, t71772, t71789, t71802, t71804, t71832, t71835, t71836, t71852, t71854, t71863)
}
