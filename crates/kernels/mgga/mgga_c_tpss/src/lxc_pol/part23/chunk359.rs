//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 359/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk359<F: Float>(t33: F, t259: F, t479: F, t1021: F, t1046: F, t1086: F, t1088: F, t1093: F, t1151: F, t1153: F, t198: F, t330: F, t826: F, t1006: F, t481: F, t57: F, t581: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t1157 = piecewise3(t480, t1151 * t1153 * t198 * t330 - t1021 + t1046 + t1086 + t1088 - t1093, t826);
    let t1162 = piecewise3(t386, t259 * t1006 / 2.0 + t826 * t33 / 2.0, t1157 * t57 / 2.0 - t481 * t581 / 2.0);
    (t1157, t1162)
}
