//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 226/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk226<F: Float>(t34: F, t365: F, t35: F, t364: F, t354: F, t122: F, t374: F, t376: F, t370: F, t368: F, t372: F, t270: F, t283: F, t61: F, t248: F, t884: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1028 = t365 * t34;
    let t1030 = 1.0 / t35 / t1028;
    let t1031 = t364 * t1030;
    let t1032 = t354 * t1031;
    let t1036 = t374 * t122 * t376;
    let t1038 = t370 * t1036 / 4608.0;
    let t1039 = t368 * t372;
    let t1040 = t364 * t1039;
    let t1041 = t354 * t1040;
    let t1043 = 1.0 / t283 / t270;
    let t1044 = t61 * t1043;
    let t1046 = t248 * t1044 * t884;
    (t1030, t1031, t1032, t1036, t1038, t1040, t1041, t1043, t1044, t1046)
}
