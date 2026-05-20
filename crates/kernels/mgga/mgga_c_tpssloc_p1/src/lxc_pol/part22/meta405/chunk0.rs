//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1704/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1704<F: Float>(t11219: F, t18206: F, t136: F, t18211: F, t3297: F, t18215: F, t6014: F, t699: F) -> (F, F, F, F, F, F, F) {
    let t18496 = t11219 * t18206;
    let t18497 = t136 * t18496;
    let t18499 = t3297 * t18211;
    let t18500 = t136 * t18499;
    let t18502 = t3297 * t18215;
    let t18503 = t136 * t18502;
    let t18505 = t699 * t6014;
    (t18496, t18497, t18499, t18500, t18502, t18503, t18505)
}
