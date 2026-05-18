//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1253/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1253<F: Float>(t1408: F, t2752: F, t2: F, t10143: F, t606: F, t1519: F, t213: F, t225: F, t794: F, t25051: F, t254: F, t853: F) -> (F, F, F, F, F, F, F) {
    let t86721 = t2752 * t1408;
    let t86730 = t2752 * t2;
    let t86770 = t10143 * t606;
    let t86873 = t213 * t1519 * t225;
    let t86893 = t794 * t1519;
    let t86988 = t25051 * t225;
    let t87013 = t853 * t254;
    (t86721, t86730, t86770, t86873, t86893, t86988, t87013)
}
