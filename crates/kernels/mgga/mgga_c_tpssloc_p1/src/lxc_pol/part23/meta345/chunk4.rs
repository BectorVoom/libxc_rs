//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1137/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1137<F: Float>(t32253: F, t59: F, t154: F, t541: F, t12289: F, t1336: F, t835: F, t1314: F, t9569: F, t2559: F, t3732: F, t12214: F, t782: F) -> (F, F, F, F, F, F, F) {
    let t39933 = t59 * t32253;
    let t39934 = t39933 * t154;
    let t39936 = F::cast_from(455.0_f64) / F::cast_from(243.0_f64) * t39934 * t541;
    let t39944 = t1336 * t12289 * t835;
    let t40005 = t9569 * t1314;
    let t40018 = t2559 * t3732;
    let t40021 = t782 * t12214;
    (t39933, t39934, t39936, t39944, t40005, t40018, t40021)
}
