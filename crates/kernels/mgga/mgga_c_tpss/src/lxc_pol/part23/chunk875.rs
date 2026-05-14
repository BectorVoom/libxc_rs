//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 875/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk875<F: Float>(t33: F, t259: F, t479: F, t1153: F, t1589: F, t198: F, t330: F, t4023: F, t6044: F, t6200: F, t6527: F, t1289: F, t1893: F, t57: F, t6221: F, t6494: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t6534 = piecewise3(t480, t1153 * t198 * t330 * t6527 - t1589 * t4023 * t6044, t6200);
    let t6539 = piecewise3(t386, t6221, -t1893 * t1289 / 2.0 + t6534 * t57 / 2.0);
    let t6540 = t6494 + t6539;
    (t6534, t6540)
}
