//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 717/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk717<F: Float>(t10570: F, t14077: F, t14154: F, t12200: F, t13801: F, t388: F, t669: F, t7933: F, t7934: F, t3047: F, t49: F, t35688: F, t7935: F) -> (F, F, F, F, F) {
    let t70149 = t10570 * t14077 * t14154;
    let t70156 = t12200 * t14077 * t13801;
    let t70169 = t7933 * t7934 * t388 * t669;
    let t70171 = t3047 * t49;
    let t70173 = t35688 * t70171 * t7935;
    (t70149, t70156, t70169, t70171, t70173)
}
