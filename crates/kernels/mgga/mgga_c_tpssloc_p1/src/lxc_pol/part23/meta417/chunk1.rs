//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1237/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1237<F: Float>(t118: F, t20800: F, t2576: F, t794: F, t21008: F, t9573: F, t20896: F, t2697: F, t13360: F, t5624: F, t1516: F, t58844: F) -> (F, F, F, F, F) {
    let t68131 = t2576 * t118 * t794 * t20800;
    let t68148 = t9573 * t21008;
    let t68195 = t2697 * t20896;
    let t68197 = t13360 * t5624;
    let t68199 = t58844 * t1516;
    (t68131, t68148, t68195, t68197, t68199)
}
