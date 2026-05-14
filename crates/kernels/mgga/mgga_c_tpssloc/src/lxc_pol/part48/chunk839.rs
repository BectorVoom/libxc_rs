//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 839/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk839<F: Float>(t31361: F, t814: F, t2627: F, t8543: F, t23168: F, t31378: F, t2553: F, t31376: F, t6552: F, t6637: F, t22893: F, t23164: F, t31377: F, t112984: F, t112988: F, t112990: F, t112992: F, t112995: F, t2633: F, t2684: F, t31394: F, t812: F, t829: F) -> (F,) {
    let t114649 = t814 * t31361;
    let t114655 = t2627 * t8543;
    let t114659 = t23168 * t31378;
    let t114663 = t6552 * t6637 * t31376 * t2553;
    let t114666 = t23164 * t22893 * t31377;
    let t114668 = -2.0 * t812 * t114649 * t829 - t812 * t31394 * t2684 + 2.0 * t812 * t114655 * t2633 + 0.76763589786250567036e-1 * t114659 - 0.16449340668482264365e-1 * t114663 + 0.16449340668482264365e-1 * t114666 + t112984 + t112988 + t112990 - t112992 + t112995;
    (t114668,)
}
