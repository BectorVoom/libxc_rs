//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 196/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk196<F: Float>(t1223: F, t31: F, t212: F, t222: F, t1189: F, t492: F, t140: F, t453: F, t73: F, t75: F, t80: F, t1007: F, t78: F) -> (F, F, F, F, F, F) {
    let t1224 = t31 * t1223;
    let t1227 = F::new(0.21341877202031537856e0) * t212 * t1224 * t222;
    let t1228 = t212 * t1189;
    let t1229 = t1228 * t492;
    let t1231 = t453 * t140;
    let t1279 = t75 * t73;
    let t1281 = F::new(132.0) * t1279 * t80;
    let t1284 = t78 * t1007;
    (t1227, t1228, t1229, t1231, t1281, t1284)
}
