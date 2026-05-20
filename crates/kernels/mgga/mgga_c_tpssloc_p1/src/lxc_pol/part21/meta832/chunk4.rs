//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2936/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2936<F: Float>(t60192: F, t60194: F, t60197: F, t60200: F, t60202: F, t60204: F, t60207: F, t60223: F, t60226: F, t60229: F, t60232: F, t60235: F) -> F {
    let t61150 = -F::new(4.0) / F::new(3.0) * t60192 + F::new(8.0) / F::new(9.0) * t60194 + t60197 - F::new(2.0) / F::new(3.0) * t60200 + F::new(4.0) / F::new(9.0) * t60202 + F::new(5.0) / F::new(81.0) * t60204 + t60207 / F::new(9.0) + t60223 / F::new(9.0) + t60226 / F::new(18.0) + F::new(2.0) / F::new(27.0) * t60229 + F::new(2.0) * t60232 + t60235;
    t61150
}
