//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1271/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1271<F: Float>(t1906: F, t82045: F, t23229: F, t81715: F, t225: F, t23228: F, t6563: F, t81597: F, t1882: F, t81686: F, t9537: F, t1883: F) -> (F, F, F, F, F, F) {
    let t82046 = t82045 * t1906;
    let t82047 = F::cast_from(0.27720185200590482541e0_f64) * t82046;
    let t82069 = t81715 * t23229;
    let t82070 = F::cast_from(0.98696044010893586188e-1_f64) * t82069;
    let t82074 = t23228 * t225;
    let t82122 = t81597 * t6563;
    let t82123 = F::cast_from(0.16220877603642232915e0_f64) * t82122;
    let t82153 = t81686 * t9537 * t1882;
    let t82154 = F::cast_from(0.13707783890401886971e-2_f64) * t82153;
    let t82218 = t82045 * t1883;
    (t82047, t82070, t82074, t82123, t82154, t82218)
}
