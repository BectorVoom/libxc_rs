//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1338/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1338<F: Float>(t1888: F, t232: F, t40909: F, t6646: F, t23177: F, t6579: F, t23143: F, t6649: F, t22999: F, t22998: F, t23185: F, t81914: F) -> (F, F, F, F, F) {
    let t82003 = t1888 * t6646 * t40909 * t232;
    let t82005 = t6579 * t23177;
    let t82011 = t23143 * t6649;
    let t82013 = t6579 * t22999;
    let t82016 = t23185 * t81914 * t22998;
    (t82003, t82005, t82011, t82013, t82016)
}
