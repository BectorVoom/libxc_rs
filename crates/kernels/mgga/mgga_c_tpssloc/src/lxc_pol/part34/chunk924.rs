//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 924/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk924<F: Float>(t23164: F, t25345: F, t10143: F, t25: F, t28: F, t870: F, t1437: F, t1864: F, t1410: F, t2240: F, t1453: F, t22470: F, t1982: F, t8944: F, t22751: F, t7692: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25346 = t23164 * t25345;
    let t25373 = t10143 * t25;
    let t25891 = t870 * t28;
    let t25927 = t10143 * t28;
    let t26012 = t1864 * t1437;
    let t26016 = t2240 * t1410;
    let t26127 = t22470 * t1453;
    let t26161 = t1982 * t8944;
    let t26184 = t22751 * t7692;
    (t25346, t25373, t25891, t25927, t26012, t26016, t26127, t26161, t26184)
}
