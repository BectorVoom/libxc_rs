//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1258/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1258<F: Float>(t1985: F, t8621: F, t90739: F, t115545: F, t1992: F, t26355: F, t22633: F, t22635: F, t31549: F, t5187: F, t33272: F, t81228: F, t81326: F, t102466: F, t120340: F, t120436: F, t120533: F, t16022: F, t16460: F, t26224: F, t26482: F, t31555: F, t31653: F, t5321: F, t5326: F, t6962: F, t7194: F, t8627: F) -> (F,) {
    let t122260 = t1985 * t90739 * t8621;
    let t122270 = t1992 * t115545 * t26355;
    let t122278 = t22633 * t22635 * t31549 * t5187;
    let t122281 = t81228 * t81326 * t33272;
    let t122285 = -t120340 - 0.82246703342411321825e-2 * t122260 + 2.0 * t16022 * t8627 - t120436 - 6.0 * t26224 * t102466 * t6962 - t120533 + 2.0 * t31653 * t5326 + 0.16449340668482264365e-1 * t122270 + 2.0 * t5321 * t31555 + 2.0 * t16460 * t8627 + 0.16449340668482264365e-1 * t122278 - 0.82246703342411321825e-2 * t122281 + 2.0 * t7194 * t26482;
    (t122285,)
}
