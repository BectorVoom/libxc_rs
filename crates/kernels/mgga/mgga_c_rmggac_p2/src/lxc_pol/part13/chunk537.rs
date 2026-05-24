//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 537/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk537<F: Float>(t511: F, t899: F, t27: F, t649: F, t794: F, t2084: F, t321: F, t2134: F, t1343: F, t265: F, t71: F) -> (F, F, F, F, F, F) {
    let t7282 = t899 * t511;
    let t7284 = t27 * t649 * t794;
    let t7285 = t7282 * t7284;
    let t7287 = t2084 * t321;
    let t7288 = t27 * t7287;
    let t7289 = t2134 * t7288;
    let t7292 = t265 * t1343 * t71;
    (t7282, t7284, t7285, t7288, t7289, t7292)
}
