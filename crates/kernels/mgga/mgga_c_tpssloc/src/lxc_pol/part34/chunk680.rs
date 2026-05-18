//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 680/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk680<F: Float>(t6553: F, t7488: F, t1880: F, t1496: F, t6581: F, t1484: F, t236: F, t1894: F, t6591: F, t1510: F, t815: F, t6605: F) -> (F, F, F, F, F, F, F) {
    let t7489 = t6553 * t7488;
    let t7490 = t1880 * t7489;
    let t7494 = t6581 * t1496;
    let t7496 = t236 * t1484;
    let t7497 = t1894 * t7496;
    let t7498 = t6591 * t7497;
    let t7500 = t815 * t1510;
    let t7501 = t6605 * t7500;
    (t7489, t7490, t7494, t7497, t7498, t7500, t7501)
}
