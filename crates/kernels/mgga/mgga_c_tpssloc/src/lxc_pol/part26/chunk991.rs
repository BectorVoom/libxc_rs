//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 991/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk991<F: Float>(t3395: F, t3400: F, t4883: F, t1164: F, t11194: F, t11272: F, t11280: F, t11288: F, t11290: F, t11296: F, t11472: F, t11475: F, t11480: F, t11482: F, t11484: F) -> (F, F) {
    let t11634 = t3400 * t3395 * t4883;
    let t11636 = F::cast_from(0.51947577317044391277e2_f64) * t1164 * t11634;
    let t11637 = -t11194 + t11272 + t11280 - t11288 + t11290 + t11296 - t11480 - t11482 - t11484 - t11472 + t11475 - t11636;
    (t11636, t11637)
}
