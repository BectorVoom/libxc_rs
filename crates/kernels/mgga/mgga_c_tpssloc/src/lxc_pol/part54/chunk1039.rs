//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1039/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1039<F: Float>(t1332: F, t8465: F, t8467: F, t1338: F, t240: F, t241: F, t1336: F) -> (F, F, F, F) {
    let t31165 = t1332 * t8465;
    let t31166 = t31165 * t8467;
    let t31169 = t1338 * t240 * t241;
    let t31170 = t1336 * t31169;
    (t31165, t31166, t31169, t31170)
}
