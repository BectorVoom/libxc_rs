//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1002/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1002<F: Float>(t115322: F, t115364: F, t115513: F, t115532: F, t115570: F, t115590: F, t115622: F, t115660: F, t1390: F, t1983: F, t533: F, t2075: F, t22479: F, t652: F) -> (F, F) {
    let t115666 = t1983 * t533 * (t115322 + t115364 + t115513 + t115532 + t115570 + t115590 + t115622 + t115660) * t1390;
    let t115669 = F::new(2.0) * t652 * t2075 * t22479;
    (t115666, t115669)
}
