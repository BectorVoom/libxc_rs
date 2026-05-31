//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 94/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk94<F: Float>(t253: F, t259: F, t144: F, t186: F, t189: F, t193: F, t202: F) -> (F, F, F) {
    let t261 = t253 * t259 + F::cast_from(1.0_f64);
    let t262 = F::ln(t261);
    let t265 = t193 * t202 * t262 - t144 + t186 + t189;
    (t261, t262, t265)
}
