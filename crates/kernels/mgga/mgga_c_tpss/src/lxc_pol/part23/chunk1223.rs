//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1223/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1223<F: Float>(t30: F, t259: F, t379: F, t20002: F, t1289: F, t1867: F, t19842: F, t3431: F, t45: F, t581: F, t5994: F, t6489: F, t1095: F, t6496: F, t6495: F, t762: F, t4216: F, t6002: F, t1695: F, t1871: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t20792 = piecewise3(t380, 0.0, t20002);
    let t20799 = piecewise3(t120, t19842, t5994 * t1289 / 2.0 + t1867 * t3431 / 2.0 + t20792 * t45 / 2.0 + t6489 * t581 / 2.0);
    let t20800 = t6496 * t1095;
    let t20802 = t6495 * t762;
    let t20806 = t6002 * t4216;
    let t20808 = t1871 * t1695;
    (t20792, t20799, t20800, t20802, t20806, t20808)
}
