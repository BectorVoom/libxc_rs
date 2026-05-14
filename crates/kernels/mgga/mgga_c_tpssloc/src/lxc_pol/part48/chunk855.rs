//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 855/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk855<F: Float>(t1880: F, t24281: F, t6553: F, t6571: F, t31420: F, t6547: F, t23171: F, t23228: F, t8547: F, t31370: F, t114866: F, t6572: F, t23204: F, t31419: F, t6562: F, t112946: F, t112949: F, t113038: F, t113041: F, t113045: F, t22974: F, t23191: F, t24325: F, t25168: F, t259: F, t26728: F, t2718: F, t2720: F, t2742: F, t31361: F, t31423: F, t6627: F, t7087: F, t798: F, t855: F, t8562: F) -> (F,) {
    let t114937 = t1880 * t6553 * t6571 * t24281;
    let t114939 = t6547 * t31420;
    let t114943 = t23171 * t23228 * t8547;
    let t114944 = 0.82246703342411321824e-2 * t114943;
    let t114945 = t6547 * t31370;
    let t114960 = t1880 * t114866 * t6572;
    let t114965 = t6562 * t23204 * t31419;
    let t114967 = -0.82246703342411321825e-2 * t114937 + 0.38381794893125283518e-1 * t114939 - t7087 * t23191 + t114944 + t112946 + t112949 + 0.38381794893125283518e-1 * t114945 + t113038 + 2.0 * t798 * t31361 * t259 + t113041 - t113045 + 2.0 * t855 * t2718 * t8562 * t2742 - 6.0 * t25168 * t26728 * t22974 + 4.0 * t6627 * t24325 - 0.16449340668482264365e-1 * t114960 + 2.0 * t31423 * t2720 + 0.82246703342411321824e-2 * t114965;
    (t114967,)
}
