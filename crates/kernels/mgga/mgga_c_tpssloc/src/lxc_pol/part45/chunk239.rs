//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 239/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk239<F: Float>(t1014: F, t363: F, t336: F, t371: F, t368: F, t1012: F, t376: F, t61: F, t890: F, t916: F, t956: F, t958: F, t963: F) -> (F, F, F, F, F, F, F) {
    let t1015 = t1014 * t363;
    let t1016 = t371 * t336;
    let t1017 = 1.0 / t1016;
    let t1018 = t368 * t1017;
    let t1019 = t1015 * t1018;
    let t1020 = t1012 * t1019;
    let t1021 = t61 * t376;
    let t1022 = -t890 + t916 + t956 + t958 - t963;
    (t1015, t1017, t1018, t1019, t1020, t1021, t1022)
}
