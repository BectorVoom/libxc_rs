//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 68/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk68<F: Float>(t153: F, t185: F, t152: F, t157: F, t182: F) -> (F, F, F, F) {
    let t186 = t153 * t185;
    let t187 = t152 * t157;
    let t189 = 0.19751673498613801407e-1 * t187 * t182;
    let t190 = f64::ln(2.0);
    let t191 = 1.0 - t190;
    (t186, t187, t189, t191)
}
