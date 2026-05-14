//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1120/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1120<F: Float>(t1139: F, t16012: F, t1136: F, t1149: F, t12557: F, t1587: F, t15931: F, t15933: F, t15944: F, t15948: F, t15953: F, t3113: F, t4296: F, t4300: F, t4323: F, t473: F, t5276: F, t5295: F) -> (F,) {
    let t16013 = t1139 * t16012;
    let t16015 = -6.0 * t1136 * t15944 + 4.0 * t1136 * t15948 + 2.0 * t1136 * t15953 - t1136 * t16013 - t1149 * t15933 - 2.0 * t12557 * t1587 + t15931 * t473 + 2.0 * t3113 * t5276 - t3113 * t5295 + 4.0 * t4296 * t4300 - 2.0 * t4296 * t4323;
    (t16015,)
}
