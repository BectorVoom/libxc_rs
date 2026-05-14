//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1214/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1214<F: Float>(t33: F, t259: F, t479: F, t20881: F, t20919: F, t3154: F, t6527: F, t1151: F, t1589: F, t1153: F, t19168: F, t19172: F, t198: F, t20002: F, t330: F, t4023: F, t4325: F, t6044: F, t1289: F, t1893: F, t20069: F, t3431: F, t57: F, t581: F, t6048: F, t6534: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t20920 = t20881 + t20919;
    let t20924 = t6527 * t3154;
    let t20929 = t1589 * t1151;
    let t20936 = piecewise3(t480, t1153 * t198 * t20920 * t330 - t1151 * t20924 * t4023 - t1589 * t19168 * t4023 + 2.0 * t19172 * t20929 * t4023 - t4023 * t4325 * t6044, t20002);
    let t20943 = piecewise3(t386, t20069, -t6048 * t1289 / 2.0 - t1893 * t3431 / 2.0 + t20936 * t57 / 2.0 - t6534 * t581 / 2.0);
    (t20920, t20924, t20929, t20936, t20943)
}
