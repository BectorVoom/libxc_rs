//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2557/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2557<F: Float>(t21938: F, t3403: F, t1117: F, t21813: F, t43969: F, t21810: F, t3264: F, t21809: F, t3315: F, t3313: F, t11275: F, t18265: F, t4781: F) -> (F, F, F, F, F) {
    let t71672 = t21938 * t3403;
    let t71697 = F::cast_from(0.62071215503128080361e4_f64) * t43969 * t21813 * t1117;
    let t71700 = F::new(2.0) * t3264 * t21810 * t1117;
    let t71701 = t21809 * t3315;
    let t71704 = F::cast_from(0.16081979498692535067e2_f64) * t3313 * t71701 * t1117;
    let t71707 = F::cast_from(0.1551780387578202009e4_f64) * t11275 * t18265 * t4781;
    (t71672, t71697, t71700, t71704, t71707)
}
