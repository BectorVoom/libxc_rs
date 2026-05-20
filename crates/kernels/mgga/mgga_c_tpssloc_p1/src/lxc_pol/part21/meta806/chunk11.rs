//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2809/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2809<F: Float>(t5611: F, t852: F, t17022: F, t814: F, t13176: F, t13390: F, t13407: F, t16673: F, t16754: F, t16762: F, t17027: F, t17041: F, t226: F, t235: F, t2617: F, t2679: F, t2728: F, t2738: F, t4166: F, t4281: F, t4282: F, t4286: F, t4288: F, t4291: F, t58340: F, t58345: F, t59328: F, t812: F, t829: F) -> (F, F) {
    let t59331 = t852 * t5611;
    let t59347 = t814 * t17022;
    let t59351 = -t17027 * t2679 * t812 + t226 * t235 * t59328 + F::new(4.0) * t2728 * t58340 * t812 + F::new(24.0) * t4281 * t4282 * t58345 - F::new(2.0) * t4291 * t59331 * t829 - F::new(2.0) * t59347 * t812 * t829 - F::new(4.0) * t13176 * t4286 - F::new(4.0) * t13176 * t4288 - F::new(4.0) * t13390 * t16762 - F::new(4.0) * t13407 * t4166 - t16673 * t2738 - F::new(2.0) * t16754 * t2617 - F::new(4.0) * t17041 * t2617;
    (t59331, t59351)
}
