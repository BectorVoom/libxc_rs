//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 206/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk206<F: Float>(t1010: F, t1011: F, t361: F, t363: F, t336: F, t371: F, t368: F, t376: F, t61: F, t122: F, t374: F, t370: F, t372: F, t364: F, t354: F, t270: F, t283: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1012 = t1010 * t1011;
    let t1013 = t361 * t361;
    let t1014 = 1.0 / t1013;
    let t1015 = t1014 * t363;
    let t1016 = t371 * t336;
    let t1017 = 1.0 / t1016;
    let t1018 = t368 * t1017;
    let t1019 = t1015 * t1018;
    let t1020 = t1012 * t1019;
    let t1021 = t61 * t376;
    let t1036 = t374 * t122 * t376;
    let t1038 = t370 * t1036 / 4608.0;
    let t1039 = t368 * t372;
    let t1040 = t364 * t1039;
    let t1041 = t354 * t1040;
    let t1043 = 1.0 / t283 / t270;
    (t1013, t1014, t1015, t1017, t1019, t1020, t1021, t1036, t1038, t1040, t1041, t1043)
}
