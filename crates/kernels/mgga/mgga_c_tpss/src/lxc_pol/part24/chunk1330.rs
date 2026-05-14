//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1330/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1330<F: Float>(t30: F, t259: F, t379: F, t70733: F, t70783: F, t70688: F, t1289: F, t13335: F, t1742: F, t20003: F, t21477: F, t3431: F, t45: F, t4579: F, t5665: F, t581: F, t6201: F, t70298: F, t18246: F, t69799: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t70784 = t70733 + t70783;
    let t70785 = piecewise3(t380, t70688, t70784);
    let t70797 = piecewise3(t120, t70298, t70785 * t45 / 2.0 + t21477 * t581 / 2.0 + t20003 * t1289 + t6201 * t3431 + t5665 * t4579 / 2.0 + t1742 * t13335 / 2.0);
    let t70800 = t18246 * t69799;
    (t70784, t70797, t70800)
}
