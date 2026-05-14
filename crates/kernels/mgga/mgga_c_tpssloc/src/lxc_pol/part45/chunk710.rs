//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 710/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk710<F: Float>(t3113: F, t6754: F, t3107: F, t6753: F, t1012: F, t1933: F, t607: F, t1937: F, t1000: F, t1025: F, t23414: F, t23419: F, t23422: F, t23425: F, t3073: F, t3098: F, t3123: F, t3143: F, t3148: F, t6717: F, t6755: F, t6765: F) -> (F,) {
    let t23433 = t3113 * t6754;
    let t23436 = t6753 * t3107;
    let t23437 = t1012 * t23436;
    let t23442 = t1933 * t607;
    let t23443 = t23442 * t1937;
    let t23445 = 0.10093189023535097714e-3 * t23414 * t1937 + t23419 * t3073 / 1152.0 - t23422 * t1000 / 54.0 + t23425 / 432.0 + t6717 * t3143 / 288.0 + t6717 * t3148 / 216.0 + t6755 * t3123 / 1536.0 + t23433 * t1025 / 768.0 - t23437 * t1025 / 144.0 - t6765 * t3098 / 1152.0 + 0.20186378047070195428e-3 * t23443;
    (t23445,)
}
