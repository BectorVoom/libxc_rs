//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 864/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk864<F: Float>(t30: F, t259: F, t379: F, t207: F, t6148: F, t1398: F, t1692: F, t198: F, t2439: F, t5590: F, t6192: F, t823: F, t1485: F, t330: F, t4023: F, t5652: F, t6185: F, t995: F, t1289: F, t1742: F, t45: F, t6160: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t6195 = t207 * t6148;
    let t6200 = -t1398 * t1692 * t5590 + t198 * t6195 * t823 + 3.0 * t2439 * t6192;
    let t6201 = piecewise3(t380, t198 * t330 * t6185 * t995 - t1485 * t4023 * t5652, t6200);
    let t6206 = piecewise3(t120, t6160, t1742 * t1289 / 2.0 + t6201 * t45 / 2.0);
    (t6195, t6200, t6201, t6206)
}
