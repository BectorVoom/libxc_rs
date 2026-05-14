//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 186/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk186<F: Float>(t1179: F, t205: F, t209: F, t28: F, t484: F, t465: F, t479: F, t31: F, t198: F, t673: F, t476: F, t77: F, t9: F, t212: F, t222: F, t492: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1180 = t205 * t1179;
    let t1184 = t209 * t209;
    let t1189 = t484 * t28;
    let t1190 = t465 * t1189;
    let t1191 = t1190 * t479;
    let t1193 = t465 * t31;
    let t1194 = t673 * t198;
    let t1195 = t1193 * t1194;
    let t1196 = t476 * t209;
    let t1223 = 1.0 / t9 / t77;
    let t1224 = t31 * t1223;
    let t1227 = 0.21341877202031537856e0 * t212 * t1224 * t222;
    let t1228 = t212 * t1189;
    let t1229 = t1228 * t492;
    (t1180, t1184, t1190, t1191, t1193, t1195, t1196, t1223, t1227, t1228, t1229)
}
