//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 865/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk865<F: Float>(t33: F, t259: F, t479: F, t1364: F, t1713: F, t1398: F, t1497: F, t1692: F, t2439: F, t5590: F, t6149: F, t6200: F, t1289: F, t1749: F, t57: F, t6206: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t6207 = t33 * t1364;
    let t6208 = t1713 * t6207;
    let t6214 = t33 * t1398;
    let t6221 = 3.0 / 2.0 * t2439 * t6208 + t1692 * t6149 * t33 / 2.0 - t1692 * t5590 * t6214 / 2.0 + t1692 * t1713 * t1497 / 2.0;
    let t6222 = piecewise3(t480, 0.0, t6200);
    let t6227 = piecewise3(t386, t6221, -t1749 * t1289 / 2.0 + t6222 * t57 / 2.0);
    let t6228 = t6206 + t6227;
    (t6207, t6214, t6222, t6228)
}
