//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1280/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1280<F: Float>(t1408: F, t2752: F, t193: F, t201: F, t7540: F, t25345: F, t82038: F, t1519: F, t213: F, t225: F, t794: F, t7480: F, t81632: F) -> (F, F, F, F, F, F) {
    let t86721 = t2752 * t1408;
    let t86736 = t193 * t201 * t7540;
    let t86870 = t82038 * t25345;
    let t86873 = t213 * t1519 * t225;
    let t86893 = t794 * t1519;
    let t86903 = t81632 * t7480;
    (t86721, t86736, t86870, t86873, t86893, t86903)
}
