//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1161/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1161<F: Float>(t1519: F, t213: F, t225: F, t794: F, t7480: F, t81632: F, t23030: F, t25035: F, t23228: F, t7479: F, t81573: F, t23012: F, t7485: F, t7489: F, t25245: F, t82031: F) -> (F, F, F, F, F, F, F, F) {
    let t86873 = t213 * t1519 * t225;
    let t86893 = t794 * t1519;
    let t86903 = t81632 * t7480;
    let t86911 = t23030 * t25035;
    let t86916 = t81573 * t23228 * t7479;
    let t86955 = t23012 * t7485;
    let t86991 = t23012 * t7489;
    let t87068 = t82031 * t25245;
    (t86873, t86893, t86903, t86911, t86916, t86955, t86991, t87068)
}
