//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 702/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk702<F: Float>(t128: F, t1330: F, t793: F, t14229: F, t7254: F, t7778: F, t7879: F, t903: F, t641: F, t7553: F, t7555: F, t2013: F, t7349: F) -> (F, F, F, F, F) {
    let t69521 = t793 * t128 * t1330;
    let t69568 = t7254 * t14229;
    let t69574 = t903 * t7778 * t7879;
    let t69583 = t7553 * t7555 * t641;
    let t69586 = t7349 * t2013;
    (t69521, t69568, t69574, t69583, t69586)
}
