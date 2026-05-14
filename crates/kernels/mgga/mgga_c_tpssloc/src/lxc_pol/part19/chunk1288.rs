//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1288/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1288<F: Float>(t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43811: F, t43816: F, t43820: F, t43823: F, t43828: F, t43808: F, t1107: F, t11223: F, t699: F) -> (F, F, F) {
    let t43831 = -80.0 / 81.0 * t43811 + 8.0 / 9.0 * t43727 - 8.0 / 3.0 * t43729 + 20.0 / 9.0 * t43734 - 112.0 / 81.0 * t43816 + t43820 - 8.0 * t43737 - 2.0 / 3.0 * t43823 - 8.0 / 9.0 * t43740 + 12.0 * t43743 + 2.0 * t43828 + 8.0 / 3.0 * t43746;
    let t43832 = t43808 + t43831;
    let t43833 = t1107 * t43832;
    let t43835 = t699 * t11223;
    (t43832, t43833, t43835)
}
