//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 875/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk875<F: Float>(t30: F, t259: F, t379: F, t1338: F, t1897: F, t6200: F, t1289: F, t1867: F, t45: F, t6160: F, t1299: F, t331: F, t136: F, t1569: F, t1875: F, t339: F, t1558: F, t1564: F, t1575: F, t444: F, t463: F, t6001: F, t6002: F, t6007: F, t6011: F, t6013: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t6486 = t1897 * t1338;
    let t6489 = piecewise3(t380, 0.0, t6200);
    let t6494 = piecewise3(t120, t6160, t1867 * t1289 / 2.0 + t6489 * t45 / 2.0);
    let t6495 = t1299 * t331;
    let t6496 = t6495 * t136;
    let t6504 = t339 * t1875 * t1569;
    let t6509 = -t6496 * t444 / 36.0 + t6001 - t6002 * t1558 / 288.0 + t6007 * t1564 / 1536.0 - t6504 * t463 / 288.0 + t6011 - t6013 * t1575 / 2304.0;
    (t6486, t6489, t6494, t6495, t6496, t6504, t6509)
}
