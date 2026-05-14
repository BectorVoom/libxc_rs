//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1097/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1097<F: Float>(t214: F, t28272: F, t1880: F, t5544: F, t6554: F, t6553: F, t6552: F, t1902: F, t5558: F, t25224: F, t7479: F, t23195: F, t5636: F, t5527: F, t23035: F, t1528: F, t17052: F, t17092: F, t1912: F, t25036: F, t25188: F, t25348: F, t259: F, t26591: F, t28265: F, t28269: F, t4147: F, t4268: F, t7517: F, t7538: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28273 = t214 * t28272;
    let t28274 = t1880 * t28273;
    let t28276 = t6554 * t5544;
    let t28277 = t6553 * t28276;
    let t28278 = t6552 * t28277;
    let t28282 = t5558 * t1902;
    let t28288 = t25224 * t7479;
    let t28289 = t6552 * t28288;
    let t28294 = t23195 * t5636;
    let t28295 = t6553 * t28294;
    let t28296 = t1880 * t28295;
    let t28298 = t6554 * t5527;
    let t28299 = t6553 * t28298;
    let t28300 = t23035 * t28299;
    let t28304 = -0.82246703342411321824e-2 * t25036 + 4.0 * t4268 * t7517 - 0.82246703342411321825e-2 * t28265 + 0.3289868133696452873e-1 * t28269 - t26591 + 0.82246703342411321825e-2 * t28274 - 0.16449340668482264365e-1 * t28278 - 2.0 * t25348 * t1528 + t28282 * t259 + 4.0 * t4147 * t7517 - 2.0 * t17092 * t1912 - 0.3289868133696452873e-1 * t28289 - 2.0 * t4147 * t7538 - t17052 * t1912 + 0.16449340668482264365e-1 * t28296 + 0.49348022005446793095e-1 * t28300 - 2.0 * t25188 * t1528;
    (t28273, t28276, t28277, t28282, t28288, t28294, t28295, t28298, t28299, t28304)
}
