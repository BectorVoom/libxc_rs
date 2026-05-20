//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1927/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1927<F: Float>(t28419: F, t6579: F, t23035: F, t23153: F, t5527: F, t6637: F, t22893: F, t28341: F, t81640: F, t1484: F, t6552: F, t87586: F) -> (F, F, F, F) {
    let t98505 = t6579 * t28419;
    let t98513 = t23035 * t6637 * t23153 * t5527;
    let t98516 = t81640 * t22893 * t28341;
    let t98520 = t6552 * t6637 * t87586 * t1484;
    (t98505, t98513, t98516, t98520)
}
