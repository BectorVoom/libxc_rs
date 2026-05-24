//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 855/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk855<F: Float>(t25518: F, t74973: F, t1627: F, t3076: F, t69195: F, t1632: F, t1635: F, t2044: F, t25607: F, t556: F, t69243: F, t2842: F, t69249: F) -> (F, F, F, F, F, F, F, F, F) {
    let t75316 = t25518 * t74973;
    let t75318 = t3076 * t1627;
    let t75319 = t69195 * t75318;
    let t75321 = t3076 * t1632;
    let t75322 = t69195 * t75321;
    let t75325 = t3076 * t1635;
    let t75326 = t25607 * t2044 * t75325;
    let t75328 = t69243 * t556;
    let t75330 = t69249 * t2842;
    (t75316, t75318, t75319, t75321, t75322, t75325, t75326, t75328, t75330)
}
