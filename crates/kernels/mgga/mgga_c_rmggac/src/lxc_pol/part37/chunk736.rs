//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 736/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk736<F: Float>(t27: F, t9157: F, t16069: F, t69609: F, t9163: F, t16074: F, t68760: F, t8450: F, t14167: F, t15291: F, t34828: F, t45468: F, t1587: F, t262: F, t3068: F, t7282: F) -> (F, F, F, F, F, F) {
    let t75260 = t27 * t9157;
    let t75262 = t69609 * t16069 * t75260;
    let t75264 = t27 * t9163;
    let t75266 = t69609 * t16074 * t75264;
    let t75268 = t8450 * t68760;
    let t75269 = t75268 * t14167;
    let t75271 = t34828 * t15291;
    let t75273 = t45468 * t15291;
    let t75277 = t7282 * t3068 * t262 * t1587;
    (t75262, t75266, t75269, t75271, t75273, t75277)
}
