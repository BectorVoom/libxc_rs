//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1157/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1157<F: Float>(t23168: F, t28330: F, t28406: F, t814: F, t234: F, t5631: F, t5593: F, t81749: F, t22690: F, t23122: F, t5544: F, t841: F, t16673: F, t6613: F, t28359: F, t838: F) -> (F, F, F, F, F, F, F) {
    let t98564 = t23168 * t28330;
    let t98592 = t814 * t28406;
    let t98598 = t234 * t5631;
    let t98618 = t81749 * t5593;
    let t98647 = t23122 * t22690 * t841 * t5544;
    let t98684 = t16673 * t6613;
    let t98690 = t28359 * t838;
    (t98564, t98592, t98598, t98618, t98647, t98684, t98690)
}
