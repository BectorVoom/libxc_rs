//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 853/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk853<F: Float>(t1587: F, t262: F, t3068: F, t7282: F, t2039: F, t2408: F, t270: F, t638: F, t41738: F, t656: F, t8941: F, t2048: F, t551: F) -> (F, F, F, F) {
    let t75277 = t7282 * t3068 * t262 * t1587;
    let t75282 = t638 * t2039 * t2408 * t270;
    let t75285 = t41738 * t656 * t8941;
    let t75298 = t2048 * t551;
    (t75277, t75282, t75285, t75298)
}
