//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 960/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk960<F: Float>(t1036: F, t6759: F, t3: F, t6740: F, t23476: F, t343: F, t23384: F, t6692: F, t1049: F, t6688: F, t1054: F, t1065: F) -> (F, F, F, F, F, F) {
    let t23560 = t6759 * t1036;
    let t23562 = t6740 * t3;
    let t23563 = t23476 * t343;
    let t23564 = t23562 * t23563;
    let t23579 = t23384 * t6692;
    let t23581 = t6688 * t1049;
    let t23587 = t1054 * t1065;
    (t23560, t23562, t23564, t23579, t23581, t23587)
}
