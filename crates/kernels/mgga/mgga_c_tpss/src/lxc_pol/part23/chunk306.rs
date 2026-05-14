//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 306/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk306<F: Float>(t837: F, t970: F, t242: F, t336: F, t363: F, t917: F, t923: F, t925: F, t931: F, t946: F, t951: F, t958: F, t964: F, t967: F) -> (F, F) {
    let t971 = t970 * t837;
    let t972 = t242 * t971;
    let t975 = -t917 * t336 / 36.0 + t923 + t925 * t931 / 288.0 + t946 * t951 / 3072.0 - t958 * t363 / 576.0 + t964 + t967 * t972 / 4608.0;
    (t972, t975)
}
