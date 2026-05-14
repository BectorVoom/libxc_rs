//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1330/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1330<F: Float>(t30: F, t259: F, t379: F, t64856: F, t10353: F, t1289: F, t1867: F, t19057: F, t1992: F, t20792: F, t3431: F, t45: F, t581: F, t5994: F, t64310: F, t6489: F, t20854: F, t219: F, t2712: F, t9738: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t68177 = piecewise3(t380, 0.0, t64856);
    let t68189 = piecewise3(t120, t64310, t68177 * t45 / 2.0 + t20792 * t581 + t6489 * t1992 / 2.0 + t19057 * t1289 / 2.0 + t5994 * t3431 + t1867 * t10353 / 2.0);
    let t68192 = t20854 * t219;
    let t68222 = t2712 * t9738;
    (t68189, t68192, t68222)
}
