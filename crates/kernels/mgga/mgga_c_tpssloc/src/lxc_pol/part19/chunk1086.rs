//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1086/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1086<F: Float>(t3795: F, t40130: F, t3792: F, t39937: F, t12282: F, t3777: F, t3809: F, t12328: F, t1333: F, t12012: F, t12351: F, t12368: F, t1307: F, t1343: F, t1354: F, t1363: F, t3719: F, t3734: F, t3790: F, t3803: F, t3851: F, t3870: F, t40114: F, t40116: F, t40119: F, t40124: F, t40126: F, t40128: F, t5248: F, t820: F) -> (F, F) {
    let t40131 = t40130 * t3795;
    let t40133 = t39937 * t3792;
    let t40138 = t3777 * t12282;
    let t40139 = t40138 * t3809;
    let t40145 = t1333 * t12328;
    let t40147 = -15.0 / 64.0 * t1363 * t12351 * t820 * t3734 * t3719 + 5.0 / 192.0 * t1363 * t3870 * t820 * t1307 * t12012 + 7.0 / 384.0 * t40114 - 35.0 / 96.0 * t40116 - t40119 * t1354 / 768.0 + 595.0 / 2592.0 * t40124 - 119.0 / 2304.0 * t40126 + 7.0 / 1152.0 * t40128 - 7.0 / 192.0 * t40131 + t3790 * t1343 * t820 * t40133 / 512.0 - 7.0 / 48.0 * t40139 - t3803 * t5248 * t12368 * t3851 / 512.0 - 595.0 / 2592.0 * t40145;
    (t40133, t40147)
}
