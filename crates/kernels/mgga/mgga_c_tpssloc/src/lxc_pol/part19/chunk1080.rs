//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1080/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1080<F: Float>(t12283: F, t12404: F, t12413: F, t12267: F, t3802: F, t3734: F, t3792: F, t12279: F, t16398: F, t12409: F, t3719: F, t12167: F, t1314: F, t9569: F, t1329: F, t12189: F, t3770: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t39971 = t12283 * t12404;
    let t39973 = t12283 * t12413;
    let t39975 = t12267 * t3802;
    let t39978 = t3792 * t3734;
    let t39983 = t16398 * t12279;
    let t39989 = t12283 * t12409;
    let t39993 = t3792 * t3719;
    let t40000 = t3792 * t12167;
    let t40005 = t9569 * t1314;
    let t40006 = t40005 * t1329;
    let t40008 = t12189 * t3770;
    (t39971, t39973, t39975, t39978, t39983, t39989, t39993, t40000, t40005, t40006, t40008)
}
