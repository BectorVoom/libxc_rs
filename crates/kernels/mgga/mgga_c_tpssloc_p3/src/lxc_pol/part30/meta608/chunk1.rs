//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2001/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2001<F: Float>(t82122: F, t794: F, t852: F, t23030: F, t23208: F, t1882: F, t81686: F, t9537: F, t213: F, t225: F, t6556: F, t81632: F) -> (F, F, F, F, F, F) {
    let t82123 = F::cast_from(0.16220877603642232915e0_f64) * t82122;
    let t82133 = t794 * t852;
    let t82147 = t23030 * t23208;
    let t82153 = t81686 * t9537 * t1882;
    let t82154 = F::cast_from(0.13707783890401886971e-2_f64) * t82153;
    let t82159 = t213 * t852 * t225;
    let t82209 = t81632 * t6556;
    (t82123, t82133, t82147, t82154, t82159, t82209)
}
