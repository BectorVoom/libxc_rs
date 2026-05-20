//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2049/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2049<F: Float>(t12019: F, t566: F, t68: F, t3700: F, t195: F, t632: F, t197: F, t636: F, t2531: F, t9892: F, t718: F, t9862: F) -> (F, F, F, F, F, F) {
    let t40590 = F::new(1.0) / t12019 / t566;
    let t40591 = t68 * t40590;
    let t40610 = t3700 * t3700;
    let t40611 = F::new(1.0) / t40610;
    let t40632 = F::new(1.0) / t195 / t632;
    let t40647 = F::new(1.0) / t197 / t636;
    let t40667 = t2531 * t9892;
    let t40673 = t718 * t9862;
    (t40591, t40611, t40632, t40647, t40667, t40673)
}
