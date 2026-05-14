//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1177/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1177<F: Float>(t118766: F, t112741: F, t112743: F, t112899: F, t22986: F, t25192: F, t112760: F, t112719: F, t1484: F, t23270: F, t2717: F, t7537: F, t1888: F, t865: F, t30634: F, t86873: F) -> (F, F, F, F, F, F, F, F) {
    let t118767 = 0.82246703342411321825e-2 * t118766;
    let t118791 = 0.82246703342411321825e-2 * t112741;
    let t118792 = 0.76763589786250567036e-1 * t112743;
    let t118802 = 0.3289868133696452873e-1 * t22986 * t112899 * t25192;
    let t118810 = 0.38381794893125283518e-1 * t112760;
    let t118814 = 0.3289868133696452873e-1 * t22986 * t23270 * t112719 * t1484;
    let t118821 = t2717 * t7537;
    let t118825 = 0.3289868133696452873e-1 * t1888 * t23270 * t118821 * t865;
    let t118828 = 0.3289868133696452873e-1 * t1888 * t86873 * t30634;
    (t118767, t118791, t118792, t118802, t118810, t118814, t118825, t118828)
}
