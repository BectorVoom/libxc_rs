//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 966/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk966<F: Float>(t1614: F, t3211: F, t1170: F, t4430: F, t1173: F, t4377: F, t724: F, t489: F, t2215: F, t4438: F, t2206: F, t10039: F, t3240: F, t4409: F, t10117: F, t4425: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12908 = t3211 * t1614;
    let t12913 = 8.0 * t1170 * t4430;
    let t12915 = 8.0 * t1173 * t4430;
    let t12916 = t4377 * t724;
    let t12918 = 2.0 * t489 * t12916;
    let t12920 = t4438 * t2215;
    let t12922 = t4438 * t2206;
    let t12924 = 4.0 * t10039;
    let t12993 = 7.0 / 72.0 * t3240 * t4409;
    let t13004 = 7.0 / 576.0 * t10117 * t4425;
    (t12908, t12913, t12915, t12918, t12920, t12922, t12924, t12993, t13004)
}
