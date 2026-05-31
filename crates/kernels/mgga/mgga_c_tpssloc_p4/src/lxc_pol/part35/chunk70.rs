//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 70/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk70<F: Float>(t153: F, t185: F, t152: F, t157: F, t182: F) -> (F, F, F, F) {
    let t186 = t153 * t185;
    let t187 = t152 * t157;
    let t189 = F::cast_from(0.19751673498613801407e-1_f64) * t187 * t182;
    let t190 = F::ln(F::cast_from(2.0_f64));
    let t191 = F::cast_from(1.0_f64) - t190;
    (t186, t187, t189, t191)
}
