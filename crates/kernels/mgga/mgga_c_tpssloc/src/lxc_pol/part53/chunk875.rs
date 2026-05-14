//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 875/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk875<F: Float>(t23185: F, t33457: F, t82074: F, t1888: F, t23270: F, t31332: F, t4300: F, t2048: F, t254: F, t1880: F, t23237: F, t33408: F, t114790: F, t23164: F, t7479: F, t114866: F, t7488: F) -> (F, F, F, F, F, F) {
    let t121444 = t23185 * t82074 * t33457;
    let t121448 = t1888 * t23270 * t31332 * t4300;
    let t121451 = t2048 * t254;
    let t121457 = t1880 * t23237 * t33408;
    let t121464 = t23164 * t114790 * t7479;
    let t121467 = t1880 * t114866 * t7488;
    (t121444, t121448, t121451, t121457, t121464, t121467)
}
