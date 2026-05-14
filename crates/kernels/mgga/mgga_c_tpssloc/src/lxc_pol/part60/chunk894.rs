//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 894/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk894<F: Float>(t1880: F, t1894: F, t214: F, t29040: F, t113005: F, t114673: F, t114689: F, t114694: F, t121536: F, t126456: F, t126472: F, t126476: F, t126477: F, t5575: F, t8560: F, t31376: F, t5544: F, t6552: F, t6637: F) -> (F, F) {
    let t127995 = t1880 * t214 * t1894 * t29040;
    let t127998 = -t126456 + t114673 - t126472 - t126476 + 0.38381794893125283518e-1 * t121536 + t126477 - t113005 + 0.82246703342411321825e-2 * t127995 - t114689 + t114694 + t5575 * t8560;
    let t128001 = t6552 * t6637 * t31376 * t5544;
    (t127998, t128001)
}
