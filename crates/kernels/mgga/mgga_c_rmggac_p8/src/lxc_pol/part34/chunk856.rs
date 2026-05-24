//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 856/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk856<F: Float>(t13911: F, t75299: F, t13916: F, t75303: F, t117: F, t1587: F, t3046: F, t2044: F, t25529: F, t75325: F, t1624: F, t262: F, t3068: F) -> (F, F, F, F, F, F) {
    let t75332 = t13911 * t75299;
    let t75334 = t13916 * t75303;
    let t75336 = t1587 * t117;
    let t75337 = t75336 * t3046;
    let t75340 = t25529 * t2044 * t75325;
    let t75344 = t25529 * t3068 * t262 * t1624;
    (t75332, t75334, t75336, t75337, t75340, t75344)
}
