//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1774/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1774<F: Float>(t120: F, t4119: F, t2645: F, t829: F, t2679: F, t4248: F, t13242: F, t4180: F, t4181: F, t4240: F, t9638: F, t2647: F) -> (F, F, F, F, F, F) {
    let t13300 = t120 * t4119;
    let t13302 = t2645 * t13300 * t829;
    let t13306 = t2645 * t4248 * t2679;
    let t13312 = t4180 * t13242 * t829;
    let t13316 = t4180 * t4181 * t2679;
    let t13320 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t9638 * t4240;
    let t13322 = t2645 * t13242 * t2647;
    (t13302, t13306, t13312, t13316, t13320, t13322)
}
