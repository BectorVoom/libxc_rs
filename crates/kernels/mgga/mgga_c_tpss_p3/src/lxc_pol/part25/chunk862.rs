//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 862/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk862<F: Float>(t46: F, t47: F, t58: F, t59: F, t7585: F, t2458: F, t78: F, t2839: F, t81: F, t2211: F, t719: F) -> (F, F, F, F, F, F) {
    let t7737 = F::cast_from(1.0_f64) / t47 / t46;
    let t7750 = F::cast_from(1.0_f64) / t59 / t58;
    let t7761 = F::cast_from(1232.0_f64) / F::cast_from(27.0_f64) * t7585;
    let t7771 = F::cast_from(1.0_f64) / t78 / t2458;
    let t7780 = F::cast_from(1.0_f64) / t81 / t2839;
    let t7813 = t2211 * t719;
    (t7737, t7750, t7761, t7771, t7780, t7813)
}
