//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 220/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk220<F: Float>(t164: F, t159: F, t688: F, t690: F, t694: F, t699: F, t167: F) -> (F, F, F, F, F) {
    let t723 = t164 * t164;
    let t724 = 1.0 / t723;
    let t725 = t159 * t724;
    let t730 = -0.1176575e1 * t688 - 0.516475e0 * t690 - 0.2103875e0 * t694 - 0.104195e0 * t699;
    let t731 = 1.0 / t167;
    (t723, t724, t725, t730, t731)
}
