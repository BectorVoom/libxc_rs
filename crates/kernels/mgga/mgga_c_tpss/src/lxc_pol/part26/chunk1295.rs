//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1295/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1295<F: Float>(t13756: F, t18454: F, t13765: F, t19476: F, t5389: F, t60738: F, t13700: F, t13687: F, t13691: F, t5373: F, t60724: F, t18436: F, t5377: F, t60707: F, t65562: F, t65571: F, t67148: F, t69510: F) -> (F,) {
    let t69512 = t18454 * t13756;
    let t69515 = t19476 * t13765;
    let t69517 = t60738 * t5389;
    let t69519 = t19476 * t13700;
    let t69521 = t18454 * t13687;
    let t69523 = t18454 * t13691;
    let t69525 = t60724 * t5373;
    let t69527 = t18436 * t5377;
    let t69529 = t69510 / 192.0 + t69512 / 192.0 - t65562 - 119.0 / 6912.0 * t60707 + t69515 / 384.0 - 7.0 / 288.0 * t69517 + t69519 / 768.0 + t69521 / 384.0 - t69523 / 768.0 - t67148 - 7.0 / 48.0 * t69525 + 7.0 / 144.0 * t69527 + t65571;
    (t69529,)
}
