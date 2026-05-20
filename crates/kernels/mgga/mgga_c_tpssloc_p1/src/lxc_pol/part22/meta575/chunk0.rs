//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2084/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2084<F: Float>(t3355: F, t427: F, t3358: F, t11292: F, t1143: F, t1124: F, t11419: F, t11282: F, t43689: F, t440: F, t43776: F, t43819: F) -> (F, F, F, F, F, F, F, F) {
    let t44175 = t3355 * t3355;
    let t44177 = t427 / t44175;
    let t44178 = t3358 * t3358;
    let t44179 = F::new(1.0) / t44178;
    let t44205 = t1143 * t11292;
    let t44214 = t1124 * t11419;
    let t44220 = t1143 * t11282;
    let t44223 = t440 * t43689;
    let t44249 = F::cast_from(0.16979925925925925926e1_f64) * t43776;
    let t44275 = F::cast_from(0.5356037037037037037e1_f64) * t43819;
    (t44177, t44179, t44205, t44214, t44220, t44223, t44249, t44275)
}
