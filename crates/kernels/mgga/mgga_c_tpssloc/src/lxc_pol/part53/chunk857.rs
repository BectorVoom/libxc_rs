//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 857/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk857<F: Float>(t25365: F, t25373: F, t16596: F, t22960: F, t4255: F, t30713: F, t4166: F, t30716: F, t112797: F, t32844: F, t13242: F, t232: F, t30714: F, t4180: F, t234: F, t240: F, t241: F, t4248: F, t776: F, t812: F, t9646: F) -> (F, F, F, F, F, F, F) {
    let t118407 = t25373 * t25365;
    let t118417 = t25373 * t16596;
    let t118440 = t22960 * t4255;
    let t118532 = t4166 * t30713;
    let t118533 = t118532 * t30716;
    let t118535 = t112797 * t32844;
    let t118539 = t30714 * t4180 * t13242 * t232;
    let t118546 = t812 * t234 * t240 * t241 * t9646 * t4248 * t776;
    (t118407, t118417, t118440, t118533, t118535, t118539, t118546)
}
