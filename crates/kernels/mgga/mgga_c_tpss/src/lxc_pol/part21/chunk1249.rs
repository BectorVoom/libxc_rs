//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1249/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1249<F: Float>(t33: F, t259: F, t479: F, t20002: F, t1289: F, t1749: F, t20069: F, t3431: F, t5686: F, t57: F, t581: F, t6222: F, t20010: F, t118: F, t1273: F, t1600: F, t1663: F, t1684: F, t1757: F, t19616: F, t19618: F, t19623: F, t19624: F, t19628: F, t19630: F, t19634: F, t19635: F, t19667: F, t4341: F, t4541: F, t544: F, t5512: F, t5702: F, t6239: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t20070 = piecewise3(t480, 0.0, t20002);
    let t20077 = piecewise3(t386, t20069, -t5686 * t1289 / 2.0 - t1749 * t3431 / 2.0 + t20070 * t57 / 2.0 - t6222 * t581 / 2.0);
    let t20078 = t20010 + t20077;
    let t20080 = -t118 * t20078 + t1273 * t6239 - t1600 * t5512 + t1663 * t5702 - t1684 * t4341 + t1757 * t4541 + t19667 * t544 + t19616 + t19618 + t19623 - t19624 - t19628 + t19630 + t19634 - t19635;
    (t20070, t20078, t20080)
}
