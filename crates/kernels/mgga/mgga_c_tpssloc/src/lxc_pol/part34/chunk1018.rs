//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1018/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1018<F: Float>(t28427: F, t6579: F, t28419: F, t22893: F, t28341: F, t81640: F, t23110: F, t23185: F, t28418: F, t23168: F, t28330: F, t234: F, t5631: F, t5593: F, t81749: F, t22690: F, t23122: F, t5544: F, t841: F) -> (F, F, F, F, F, F, F, F) {
    let t98490 = t6579 * t28427;
    let t98505 = t6579 * t28419;
    let t98516 = t81640 * t22893 * t28341;
    let t98549 = t23185 * t23110 * t28418;
    let t98564 = t23168 * t28330;
    let t98598 = t234 * t5631;
    let t98618 = t81749 * t5593;
    let t98647 = t23122 * t22690 * t841 * t5544;
    (t98490, t98505, t98516, t98549, t98564, t98598, t98618, t98647)
}
