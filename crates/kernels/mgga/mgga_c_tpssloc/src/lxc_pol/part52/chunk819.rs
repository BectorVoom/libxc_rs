//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 819/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk819<F: Float>(t1894: F, t1902: F, t214: F, t1880: F, t235: F, t8347: F, t226: F) -> (F, F, F, F) {
    let t8356 = t1894 * t1902;
    let t8357 = t214 * t8356;
    let t8359 = 0.16449340668482264365e-1 * t1880 * t8357;
    let t8360 = t235 * t8347;
    let t8362 = t226 * t8360 + t8359;
    (t8356, t8357, t8360, t8362)
}
