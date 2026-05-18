//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1286/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1286<F: Float>(t22690: F, t5527: F, t81792: F, t841: F, t236: F, t5584: F, t23109: F, t2632: F, t81914: F, t23110: F, t232: F, t5611: F) -> (F, F, F, F) {
    let t98774 = t81792 * t22690 * t841 * t5527;
    let t98779 = t236 * t5584;
    let t98782 = t23109 * t81914 * t98779 * t2632;
    let t98787 = t23109 * t23110 * t236 * t5611 * t232;
    (t98774, t98779, t98782, t98787)
}
