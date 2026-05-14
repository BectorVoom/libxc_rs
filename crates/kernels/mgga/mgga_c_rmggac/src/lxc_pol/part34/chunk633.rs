//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 633/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk633<F: Float>(t14031: F, t14367: F, t14035: F, t1326: F, t14147: F, t3057: F, t14150: F, t290: F, t35253: F, t10570: F, t14077: F, t14154: F, t12200: F, t13801: F, t388: F, t669: F, t7933: F, t7934: F) -> (F, F, F, F, F, F, F) {
    let t70123 = t14031 * t14367;
    let t70124 = t70123 * t14035;
    let t70127 = t14147 * t1326 * t3057;
    let t70130 = t70127 * t35253 * t290 * t14150;
    let t70149 = t10570 * t14077 * t14154;
    let t70156 = t12200 * t14077 * t13801;
    let t70169 = t7933 * t7934 * t388 * t669;
    (t70123, t70124, t70127, t70130, t70149, t70156, t70169)
}
