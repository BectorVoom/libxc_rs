//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2461/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2461<F: Float>(t3355: F, t427: F, t3358: F, t3368: F, t3400: F, t3375: F, t11292: F, t1143: F, t3324: F, t3331: F, t1124: F, t11419: F) -> (F, F, F, F, F, F, F) {
    let t44175 = t3355 * t3355;
    let t44177 = t427 / t44175;
    let t44178 = t3358 * t3358;
    let t44179 = F::new(1.0) / t44178;
    let t44188 = t3368 * t3400;
    let t44202 = t3368 * t3375;
    let t44205 = t1143 * t11292;
    let t44211 = t3324 * t3331;
    let t44214 = t1124 * t11419;
    (t44177, t44179, t44188, t44202, t44205, t44211, t44214)
}
