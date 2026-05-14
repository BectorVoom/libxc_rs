//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 637/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk637<F: Float>(t212: F, t28: F, t3144: F, t4071: F, t672: F, t14015: F, t14371: F, t16059: F, t511: F, t7231: F, t2046: F, t2049: F, t2169: F, t2039: F, t2153: F, t270: F, t638: F) -> (F, F, F, F, F) {
    let t70328 = t672 * t212 * t4071 * t28 * t3144;
    let t70330 = t14371 * t14015;
    let t70336 = t511 * t16059;
    let t70337 = t7231 * t70336;
    let t70358 = t2046 * t2049 * t2169;
    let t70365 = t638 * t2039 * t2153 * t270;
    (t70328, t70330, t70337, t70358, t70365)
}
