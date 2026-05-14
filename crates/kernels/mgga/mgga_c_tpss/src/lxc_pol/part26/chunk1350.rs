//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1350/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1350<F: Float>(t30: F, t259: F, t379: F, t70784: F, t1289: F, t13335: F, t1867: F, t20792: F, t21988: F, t3431: F, t45: F, t4579: F, t581: F, t5994: F, t6489: F, t70298: F, t22045: F, t5570: F, t1107: F, t22037: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t73143 = piecewise3(t380, 0.0, t70784);
    let t73155 = piecewise3(t120, t70298, t73143 * t45 / 2.0 + t21988 * t581 / 2.0 + t20792 * t1289 + t6489 * t3431 + t5994 * t4579 / 2.0 + t1867 * t13335 / 2.0);
    let t73171 = t22045 * t5570;
    let t73191 = t1107 * t22037;
    (t73155, t73171, t73191)
}
