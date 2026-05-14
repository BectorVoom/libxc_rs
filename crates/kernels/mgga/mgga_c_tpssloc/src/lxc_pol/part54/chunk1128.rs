//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1128/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1128<F: Float>(t10143: F, t7109: F, t111: F, t7415: F, t25: F, t40772: F, t1408: F, t2752: F, t2: F, t606: F, t1519: F, t213: F, t225: F, t794: F, t25051: F, t254: F, t853: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t84800 = t7109 * t10143;
    let t85416 = t7415 * t111;
    let t86716 = t40772 * t25;
    let t86721 = t2752 * t1408;
    let t86730 = t2752 * t2;
    let t86770 = t10143 * t606;
    let t86873 = t213 * t1519 * t225;
    let t86893 = t794 * t1519;
    let t86988 = t25051 * t225;
    let t87013 = t853 * t254;
    (t84800, t85416, t86716, t86721, t86730, t86770, t86873, t86893, t86988, t87013)
}
