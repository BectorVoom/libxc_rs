//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1136/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1136<F: Float>(t17974: F, t803: F, t2391: F, t5559: F, t2395: F, t17945: F, t17948: F, t17950: F, t17952: F, t17957: F, t17962: F, t17965: F, t17967: F, t17969: F, t17972: F, t219: F, t5563: F) -> (F, F, F, F) {
    let t17975 = t17974 * t803;
    let t17976 = 7.0 / 288.0 * t17975;
    let t17977 = t5559 * t2391;
    let t17979 = t5559 * t2395;
    let t17981 = t17945 + t17948 + t17950 / 16.0 - t17952 / 48.0 + t17957 / 768.0 + t17962 + t17965 / 192.0 - t17967 / 1536.0 - t17969 / 1536.0 + t17972 + t17976 + 5.0 / 384.0 * t17977 - t17979 / 384.0;
    let t17982 = param_beta * t17981;
    let t17984 = t5563 * t219;
    (t17975, t17981, t17982, t17984)
}
