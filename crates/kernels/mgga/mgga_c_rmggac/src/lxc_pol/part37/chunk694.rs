//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 694/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk694<F: Float>(t14327: F, t6444: F, t14308: F, t3839: F, t14173: F, t3814: F, t35589: F, t664: F, t305: F, t3851: F, t68737: F, t3046: F, t874: F) -> (F, F, F, F, F, F, F) {
    let t69166 = t6444 * t14327;
    let t69171 = t3839 * t14308;
    let t69176 = t3814 * t14173;
    let t69179 = t35589 * t664;
    let t69181 = F::cast_from(0.2927036860455597649e0_f64) * t305 * t69179;
    let t69182 = t3851 * t68737;
    let t69183 = F::cast_from(0.23948483403727617128e0_f64) * t69182;
    let t69184 = t874 * t3046;
    (t69166, t69171, t69176, t69179, t69181, t69183, t69184)
}
