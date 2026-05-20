//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3004/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3004<F: Float>(t62729: F, t62730: F, t62732: F, t62733: F, t62736: F, t62737: F, t62739: F, t62754: F, t17152: F, t42972: F, t973: F, t10876: F, t13969: F, t17983: F) -> (F, F, F) {
    let t62757 = t62729 + t62730 + t62732 + t62733 + t62736 + t62737 + t62739 + t62754;
    let t62766 = t973 * t42972 * t17152;
    let t62778 = t10876 * t13969 * t17983;
    (t62757, t62766, t62778)
}
