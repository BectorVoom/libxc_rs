//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1996/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1996<F: Float>(t213: F, t6589: F, t9223: F, t6593: F, t22715: F, t229: F, t805: F, t1891: F, t192: F, t22690: F, t80881: F, t841: F) -> (F, F, F, F, F) {
    let t81933 = t9223 * t6589 * t213;
    let t81934 = t81933 * t6593;
    let t81942 = t22715 * t229;
    let t81943 = t81942 * t805;
    let t81954 = t80881 * t1891 * t192 * t22690 * t841;
    (t81933, t81934, t81942, t81943, t81954)
}
