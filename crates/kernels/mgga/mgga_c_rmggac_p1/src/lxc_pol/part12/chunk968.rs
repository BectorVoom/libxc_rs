//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 968/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk968<F: Float>(t1240: F, t1971: F, t515: F, t570: F, t7230: F, t2289: F, t36542: F, t34884: F, t8668: F, t8831: F, t8836: F, t8843: F) -> (F, F, F, F, F, F) {
    let t40554 = t7230 * t1971 * t515 * t570 * t1240;
    let t40556 = t36542 * t2289;
    let t40558 = t34884 * t8668;
    let t40559 = F::cast_from(0.24829349937757072982e-4_f64) * t40558;
    let t40560 = t34884 * t8831;
    let t40561 = F::cast_from(0.74488049813271218946e-4_f64) * t40560;
    let t40562 = t34884 * t8836;
    let t40563 = F::cast_from(0.74488049813271218946e-4_f64) * t40562;
    let t40564 = t34884 * t8843;
    (t40554, t40556, t40559, t40561, t40563, t40564)
}
