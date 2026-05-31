//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 590/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk590<F: Float>(t2349: F, t3308: F, t1246: F, t73: F, t2377: F, t242: F, t527: F, t525: F, t1242: F, t339: F, t789: F) -> (F, F, F, F, F) {
    let t3310 = F::cast_from(0.10843581300301739842e-1_f64) * t3308 * t2349;
    let t3319 = t73 * t1246;
    let t3338 = t2377 * t527 * t242;
    let t3340 = F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t525 * t3338;
    let t3342 = t339 * t1242 * t789;
    (t3310, t3319, t3338, t3340, t3342)
}
