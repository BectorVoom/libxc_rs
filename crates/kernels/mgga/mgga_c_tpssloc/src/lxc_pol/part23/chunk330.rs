//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 330/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk330<F: Float>(t535: F, t792: F, t795: F, t541: F, t801: F, t544: F, t68: F) -> (F, F, F) {
    let t1322 = 0.41666666666666666666e-3 * t792 * t535 * t795;
    let t1327 = 7.0 / 288.0 * t801 * t541;
    let t1336 = t544 * t68;
    (t1322, t1327, t1336)
}
