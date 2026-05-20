//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 821/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk821<F: Float>(t3241: F, t76: F, t107: F, t2585: F, t655: F, t93: F, t94: F, t101: F, t102: F, t195: F, t40: F, t197: F, t52: F) -> (F, F, F, F, F, F, F) {
    let t9330 = F::new(1.0) / t76 / t3241;
    let t9358 = F::new(154.0) / F::new(27.0) * t2585 * t107;
    let t9364 = t655 * t655;
    let t9365 = F::new(1.0) / t9364;
    let t9383 = t94 * t93;
    let t9384 = F::new(1.0) / t9383;
    let t9397 = t102 * t101;
    let t9398 = F::new(1.0) / t9397;
    let t9427 = F::new(1.0) / t195 / t40;
    let t9438 = F::new(1.0) / t197 / t52;
    (t9330, t9358, t9365, t9384, t9398, t9427, t9438)
}
