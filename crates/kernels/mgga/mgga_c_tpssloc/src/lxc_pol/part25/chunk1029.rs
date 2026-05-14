//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1029/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1029<F: Float>(t25119: F, t841: F, t9616: F, t1891: F, t192: F, t22690: F, t80881: F, t244: F, t6546: F, t2606: F, t1878: F, t845: F, t10033: F, t2230: F, t23076: F, t213: F) -> (F, F, F, F, F) {
    let t81949 = t25119 * t841 * t9616;
    let t81954 = t80881 * t1891 * t192 * t22690 * t841;
    let t81956 = t6546 * t244;
    let t81957 = t81956 * t2606;
    let t81959 = t1878 * t845;
    let t81960 = t81959 * t10033;
    let t81962 = t2230 * t23076;
    let t81963 = t81962 * t213;
    (t81949, t81954, t81957, t81960, t81963)
}
