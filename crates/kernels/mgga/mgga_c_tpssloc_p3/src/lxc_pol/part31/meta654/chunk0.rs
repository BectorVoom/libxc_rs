//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1934/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1934<F: Float>(t23097: F, t232: F, t67783: F, t815: F, t16888: F, t23146: F, t16969: F, t25146: F, t4236: F, t23053: F, t5614: F, t16859: F, t6614: F) -> (F, F, F, F, F, F) {
    let t98672 = t23097 * t815 * t67783 * t232;
    let t98674 = t23146 * t16888;
    let t98676 = t23146 * t16969;
    let t98678 = t25146 * t4236;
    let t98680 = t23053 * t5614;
    let t98682 = t6614 * t16859;
    (t98672, t98674, t98676, t98678, t98680, t98682)
}
