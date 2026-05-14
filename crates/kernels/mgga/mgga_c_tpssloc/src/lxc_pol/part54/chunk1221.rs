//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1221/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1221<F: Float>(t23185: F, t33457: F, t82074: F, t1888: F, t23270: F, t31332: F, t4300: F, t2048: F, t254: F, t225: F, t33414: F, t1880: F, t23237: F, t33408: F, t114795: F, t114811: F, t114815: F, t118640: F, t118810: F, t118814: F, t118825: F, t1528: F, t25170: F, t2597: F, t26729: F, t33399: F, t866: F) -> (F,) {
    let t121444 = t23185 * t82074 * t33457;
    let t121448 = t1888 * t23270 * t31332 * t4300;
    let t121451 = t2048 * t254;
    let t121454 = t33414 * t225;
    let t121457 = t1880 * t23237 * t33408;
    let t121462 = 0.41123351671205660912e-2 * t114795 - 0.82246703342411321825e-2 * t121444 + 0.16449340668482264365e-1 * t121448 - t114811 * t1528 - 6.0 * t121451 * t25170 - t121454 * t866 - t118810 - t114815 - 0.82246703342411321825e-2 * t121457 - t2597 * t33399 - 6.0 * t118640 * t26729 + t118814 + t118825;
    (t121462,)
}
