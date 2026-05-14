//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1225/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1225<F: Float>(t114696: F, t1484: F, t6552: F, t6637: F, t31376: F, t4119: F, t23110: F, t23185: F, t33379: F, t1888: F, t232: F, t6646: F, t92745: F, t112990: F, t112995: F, t114659: F, t114666: F, t118725: F, t118728: F, t118730: F, t118735: F, t118736: F, t118737: F, t1499: F, t31397: F, t33396: F, t808: F) -> (F,) {
    let t121517 = t6552 * t6637 * t114696 * t1484;
    let t121521 = t6552 * t6637 * t31376 * t4119;
    let t121524 = t23185 * t23110 * t33379;
    let t121528 = t1888 * t6646 * t92745 * t232;
    let t121531 = t1499 * t31397 + 0.38381794893125283518e-1 * t114659 + 0.82246703342411321824e-2 * t114666 - 0.16449340668482264365e-1 * t121517 - 0.16449340668482264365e-1 * t121521 - t118725 + t118728 + 0.41123351671205660912e-2 * t121524 - 0.82246703342411321825e-2 * t121528 + t118730 - t118735 + t808 * t33396 + t112990 - t118736 + t112995 - t118737;
    (t121531,)
}
