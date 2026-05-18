//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1319/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1319<F: Float>(t112760: F, t112719: F, t1484: F, t22986: F, t23270: F, t2717: F, t7537: F, t1888: F, t865: F, t30634: F, t86873: F, t112943: F, t6562: F, t7488: F) -> (F, F, F, F, F) {
    let t118810 = F::new(0.38381794893125283518e-1) * t112760;
    let t118814 = F::new(0.3289868133696452873e-1) * t22986 * t23270 * t112719 * t1484;
    let t118821 = t2717 * t7537;
    let t118825 = F::new(0.3289868133696452873e-1) * t1888 * t23270 * t118821 * t865;
    let t118828 = F::new(0.3289868133696452873e-1) * t1888 * t86873 * t30634;
    let t118830 = t6562 * t112943 * t7488;
    (t118810, t118814, t118825, t118828, t118830)
}
