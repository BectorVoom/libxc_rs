//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2882/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2882<F: Float>(t136: F, t2826: F, t59676: F, t59661: F, t59663: F, t59665: F, t59670: F, t59674: F, t59678: F, t60186: F, t60189: F, t60192: F, t60194: F, t60197: F, t60200: F, t60202: F, t60204: F) -> (F, F) {
    let t60207 = t136 * t2826 * t59676;
    let t60214 = F::new(0.16504875e0) * t60186 + F::new(0.198684e1) * t60189 + F::new(0.72462e1) * t59661 + F::new(0.66228e0) * t60192 - F::new(0.44152e0) * t60194 - F::new(0.49671e0) * t60197 + F::new(0.33114e0) * t60200 - F::new(0.22076e0) * t60202 - F::cast_from(0.30661111111111111112e-1_f64) * t60204 - F::new(0.5519e-1) * t60207 - F::cast_from(0.40256666666666666667e0_f64) * t59663 + F::cast_from(0.13418888888888888889e0_f64) * t59665 - F::cast_from(0.40256666666666666666e0_f64) * t59670 - F::cast_from(0.20128333333333333333e0_f64) * t59674 - F::cast_from(0.40256666666666666666e0_f64) * t59678;
    (t60207, t60214)
}
