//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1229/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1229<F: Float>(t114797: F, t1484: F, t22986: F, t23270: F, t33448: F, t81591: F, t1888: F, t33457: F, t82159: F, t1880: F, t214: F, t225: F, t258: F, t26653: F, t114760: F, t114762: F, t118526: F, t118626: F, t118630: F, t118633: F, t1527: F, t23281: F, t25168: F, t25199: F, t26728: F, t2718: F, t31399: F, t7516: F, t7830: F, t855: F, t865: F, t92394: F) -> (F,) {
    let t121367 = t22986 * t23270 * t114797 * t1484;
    let t121371 = t81591 * t33448;
    let t121382 = t1888 * t82159 * t33457;
    let t121391 = t1880 * t214 * t26653 * t225 * t258;
    let t121393 = 0.16449340668482264365e-1 * t121367 + 2.0 * t23281 * t7830 - 0.38381794893125283518e-1 * t121371 - t118526 + 24.0 * t25168 * t92394 * t7516 * t865 - t118626 - 6.0 * t25168 * t26728 * t25199 + t114760 + t118630 - t118633 - 0.38381794893125283518e-1 * t114762 + 0.16449340668482264365e-1 * t121382 + 2.0 * t855 * t2718 * t31399 * t1527 + 0.82246703342411321825e-2 * t121391;
    (t121393,)
}
