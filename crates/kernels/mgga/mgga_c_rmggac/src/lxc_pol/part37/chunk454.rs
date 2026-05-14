//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 454/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk454<F: Float>(t14011: F, t335: F, t3120: F, t354: F, t3112: F, t457: F, t201: F, t1173: F, t3116: F) -> (F, F, F, F, F, F, F) {
    let t14012 = t14011 * t335;
    let t14013 = t3120 * t14012;
    let t14015 = t14011 * t354;
    let t14016 = t3120 * t14015;
    let t14018 = t3112 * t457;
    let t14019 = t14018 * t201;
    let t14020 = t1173 * t3116;
    (t14012, t14013, t14015, t14016, t14018, t14019, t14020)
}
