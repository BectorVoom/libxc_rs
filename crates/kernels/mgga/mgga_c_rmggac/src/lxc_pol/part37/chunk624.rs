//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 624/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk624<F: Float>(t14229: F, t7254: F, t7778: F, t7879: F, t903: F, t641: F, t7553: F, t7555: F, t2013: F, t7349: F, t2000: F, t838: F, t14113: F, t68621: F, t68523: F, t7229: F) -> (F, F, F, F, F, F, F) {
    let t69568 = t7254 * t14229;
    let t69574 = t903 * t7778 * t7879;
    let t69583 = t7553 * t7555 * t641;
    let t69586 = t7349 * t2013;
    let t69588 = t2000 * t838;
    let t69594 = t14113 * t68621;
    let t69598 = t7229 * t68523;
    (t69568, t69574, t69583, t69586, t69588, t69594, t69598)
}
