//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1161/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1161<F: Float>(t12379: F, t3799: F, t12384: F, t3777: F, t3795: F, t3792: F, t39937: F, t12282: F, t3809: F, t12328: F, t1333: F, t12012: F, t12351: F, t12368: F, t1307: F, t1343: F, t1354: F, t1363: F, t3719: F, t3734: F, t3790: F, t3803: F, t3851: F, t3870: F, t40114: F, t40116: F, t40119: F, t40124: F, t40126: F, t5248: F, t820: F) -> (F, F) {
    let t40128 = t3799 * t12379;
    let t40130 = t3777 * t12384;
    let t40131 = t40130 * t3795;
    let t40133 = t39937 * t3792;
    let t40138 = t3777 * t12282;
    let t40139 = t40138 * t3809;
    let t40145 = t1333 * t12328;
    let t40147 = -F::cast_from(15.0_f64) / F::cast_from(64.0_f64) * t1363 * t12351 * t820 * t3734 * t3719 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t1363 * t3870 * t820 * t1307 * t12012 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t40114 - F::cast_from(35.0_f64) / F::cast_from(96.0_f64) * t40116 - t40119 * t1354 / F::cast_from(768.0_f64) + F::cast_from(595.0_f64) / F::cast_from(2592.0_f64) * t40124 - F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t40126 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t40128 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t40131 + t3790 * t1343 * t820 * t40133 / F::cast_from(512.0_f64) - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t40139 - t3803 * t5248 * t12368 * t3851 / F::cast_from(512.0_f64) - F::cast_from(595.0_f64) / F::cast_from(2592.0_f64) * t40145;
    (t40133, t40147)
}
