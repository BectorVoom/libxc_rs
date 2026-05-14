//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 893/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk893<F: Float>(t46: F, t47: F, t58: F, t59: F, t7585: F, t2458: F, t78: F, t2839: F, t81: F, t116: F, t2053: F) -> (F, F, F, F, F, F) {
    let t7737 = 1.0 / t47 / t46;
    let t7750 = 1.0 / t59 / t58;
    let t7761 = 1232.0 / 27.0 * t7585;
    let t7771 = 1.0 / t78 / t2458;
    let t7780 = 1.0 / t81 / t2839;
    let t7798 = t2053 * t116;
    (t7737, t7750, t7761, t7771, t7780, t7798)
}
