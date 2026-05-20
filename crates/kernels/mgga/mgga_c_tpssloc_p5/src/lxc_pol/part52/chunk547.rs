//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 547/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk547<F: Float>(t1229: F, t154: F, t636: F, t2296: F, t1094: F, t1098: F, t1097: F, t419: F, t409: F, t407: F, t410: F, t3236: F) -> (F, F, F, F, F, F, F) {
    let t3240 = t154 * t1229;
    let t3241 = t636 * t636;
    let t3242 = F::new(1.0) / t3241;
    let t3247 = F::new(1.0) / t2296;
    let t3259 = t1094 * t1098;
    let t3262 = t1097 * t419;
    let t3263 = F::new(1.0) / t3262;
    let t3264 = t409 * t3263;
    let t3270 = F::new(1.0) / t410 / t407;
    let t3274 = F::new(4.0) / F::new(9.0) * t3236;
    (t3240, t3242, t3247, t3259, t3264, t3270, t3274)
}
