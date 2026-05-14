//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 435/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk435<F: Float>(t2250: F, t31: F, t65: F, t608: F, t628: F, t36: F, t365: F, sigma0: F) -> (F, F, F, F, F) {
    let t2251 = t31 * t2250;
    let t2252 = t2251 * t65;
    let t2255 = t608 * t628;
    let t2261 = 1.0 / t36 / t365;
    let t2262 = sigma0 * t2261;
    (t2251, t2252, t2255, t2261, t2262)
}
