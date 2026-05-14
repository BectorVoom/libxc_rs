//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 527/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk527<F: Float>(t2046: F, t2323: F, t3047: F, t2060: F, t2367: F, t739: F, t2131: F, t2415: F, t2010: F, t13819: F, t2416: F, t14117: F, t2304: F, t14116: F, t2868: F, t3072: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15047 = t2046 * t3047 * t2323;
    let t15049 = t2060 * t2367;
    let t15050 = t739 * t15049;
    let t15051 = 0.2993560425465952141e-1 * t15050;
    let t15061 = t2415 * t2131;
    let t15062 = t2010 * t15061;
    let t15064 = t13819 * t2416;
    let t15067 = t14117 * t2304;
    let t15068 = t14116 * t15067;
    let t15070 = t2868 * t3072;
    (t15047, t15049, t15051, t15061, t15062, t15064, t15067, t15068, t15070)
}
