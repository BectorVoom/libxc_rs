//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 712/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk712<F: Float>(t6657: F, t829: F, t235: F, t6624: F, t1909: F, t226: F, t6636: F, t6641: F, t6645: F, t6650: F, t6654: F, t808: F, t812: F, t858: F) -> (F, F, F, F) {
    let t6658 = t6657 * t829;
    let t6660 = t235 * t6624;
    let t6662 = -t6636 - 0.16449340668482264365e-1 * t6641 - t6645 - 0.82246703342411321825e-2 * t6650 + 0.82246703342411321825e-2 * t6654 + t808 * t1909 - t812 * t6658 + t226 * t6660;
    let t6663 = t858 * t6662;
    (t6658, t6660, t6662, t6663)
}
