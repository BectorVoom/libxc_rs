//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 900/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk900<F: Float>(t12571: F, t6489: F, t1860: F, t1865: F, t22544: F, t22549: F, t22551: F, t26009: F, t26013: F, t26016: F, t26021: F, t26025: F, t26028: F, t26045: F, t26048: F, t6486: F, t6492: F, t6506: F, t6510: F, t7428: F, t7442: F, t7446: F) -> (F,) {
    let t26051 = t12571 * t6489;
    let t26054 = -5.0 * t22544 * t26009 - 5.0 / 3.0 * t22549 * t26013 - 5.0 / 3.0 * t26016 * t22551 - t6486 * t7446 / 6.0 - t1860 * t26021 / 6.0 - t1860 * t26025 / 6.0 - t26028 * t1865 / 6.0 - t7428 * t6506 / 6.0 - t7428 * t6510 / 6.0 - t6486 * t7442 / 6.0 - t1860 * t26045 / 6.0 - t1860 * t26048 / 6.0 + 5.0 / 6.0 * t26051 * t6492;
    (t26054,)
}
