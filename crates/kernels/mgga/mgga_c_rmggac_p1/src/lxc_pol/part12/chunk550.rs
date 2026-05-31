//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 550/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk550<F: Float>(t31: F, t357: F, t2046: F, t2050: F, t2131: F, t931: F, t668: F, t934: F) -> (F, F, F, F) {
    let t7393 = t357 * t31;
    let t7395 = t2046 * t2050 * t7393;
    let t7397 = t931 * t2131;
    let t7398 = F::cast_from(0.2363e1_f64) * t7397;
    let t7399 = t934 * t668;
    (t7393, t7395, t7398, t7399)
}
