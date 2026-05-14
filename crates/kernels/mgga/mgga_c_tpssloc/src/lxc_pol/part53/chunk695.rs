//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 695/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk695<F: Float>(t23204: F, t6555: F, t23164: F, t6572: F, t6562: F, t212: F, t252: F, t6554: F, t23171: F, t23168: F, t6556: F, t6547: F, t6573: F, t214: F, t852: F, t6568: F) -> (F, F, F, F, F, F, F, F) {
    let t23205 = t23204 * t6555;
    let t23206 = t23164 * t23205;
    let t23207 = 0.16449340668482264365e-1 * t23206;
    let t23208 = t23204 * t6572;
    let t23209 = t6562 * t23208;
    let t23228 = t212 * t252;
    let t23229 = t23228 * t6554;
    let t23230 = t23171 * t23229;
    let t23232 = t23168 * t6556;
    let t23233 = 0.76763589786250567036e-1 * t23232;
    let t23235 = t6547 * t6573;
    let t23236 = 0.38381794893125283518e-1 * t23235;
    let t23237 = t214 * t852;
    let t23249 = t6547 * t6568;
    (t23207, t23209, t23228, t23230, t23233, t23236, t23237, t23249)
}
