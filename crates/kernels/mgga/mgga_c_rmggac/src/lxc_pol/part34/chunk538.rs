//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 538/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk538<F: Float>(t3046: F, t352: F, t1326: F, t14309: F, t2123: F, t36: F, t2079: F, t262: F, t13996: F, t305: F, t14003: F, t5148: F) -> (F, F, F, F, F) {
    let t14310 = t3046 * t352;
    let t14312 = t14309 * t1326 * t14310;
    let t14314 = t36 * t2123;
    let t14316 = t2079 * t262 * t14314;
    let t14319 = F::cast_from(0.2993560425465952141e-1_f64) * t305 * t13996;
    let t14324 = F::cast_from(0.5987120850931904282e-1_f64) * t5148 * t14003;
    (t14312, t14314, t14316, t14319, t14324)
}
