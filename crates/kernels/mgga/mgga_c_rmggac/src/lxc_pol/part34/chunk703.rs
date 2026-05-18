//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 703/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk703<F: Float>(t2000: F, t838: F, t14113: F, t68621: F, t68523: F, t7229: F, t14233: F, t14161: F, t221: F, t1966: F) -> (F, F, F, F, F, F) {
    let t69588 = t2000 * t838;
    let t69594 = t14113 * t68621;
    let t69598 = t7229 * t68523;
    let t69599 = t69598 * t14233;
    let t69608 = t14161 * t221;
    let t69609 = t1966 * t69608;
    (t69588, t69594, t69598, t69599, t69608, t69609)
}
