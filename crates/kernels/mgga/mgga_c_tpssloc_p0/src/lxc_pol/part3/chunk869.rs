//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 869/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk869<F: Float>(t2333: F, t626: F, t2359: F, t655: F, t93: F, t94: F, t101: F, t102: F, t195: F, t40: F, t197: F, t52: F) -> (F, F, F, F, F, F, F) {
    let t9361 = t626 * t2333;
    let t9363 = t626 * t2359;
    let t9364 = t655 * t655;
    let t9365 = F::new(1.0) / t9364;
    let t9383 = t94 * t93;
    let t9384 = F::new(1.0) / t9383;
    let t9397 = t102 * t101;
    let t9398 = F::new(1.0) / t9397;
    let t9427 = F::new(1.0) / t195 / t40;
    let t9438 = F::new(1.0) / t197 / t52;
    (t9361, t9363, t9365, t9384, t9398, t9427, t9438)
}
