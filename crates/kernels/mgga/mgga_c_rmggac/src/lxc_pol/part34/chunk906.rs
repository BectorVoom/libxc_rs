//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 906/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk906<F: Float>(t25854: F, t75848: F, t27048: F, t75851: F, t14305: F, t75303: F, t1326: F, t14309: F, t2048: F, t570: F, t2079: F, t2367: F, t262: F, t265: F) -> (F, F, F, F, F) {
    let t76222 = F::new(0.17961362552795712846e0) * t25854 * t75848;
    let t76224 = F::new(0.17961362552795712846e0) * t27048 * t75851;
    let t76228 = t14305 * t75303;
    let t76232 = t14309 * t1326 * t2048 * t570;
    let t76236 = t2079 * t262 * t265 * t2367;
    (t76222, t76224, t76228, t76232, t76236)
}
