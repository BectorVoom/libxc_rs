//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 451/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk451<F: Float>(t33: F, t259: F, t479: F, t1153: F, t1402: F, t1507: F, t1521: F, t1547: F, t1549: F, t1553: F, t1589: F, t198: F, t330: F, t1289: F, t1497: F, t481: F, t57: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t1594 = piecewise3(t480, t1153 * t1589 * t198 * t330 - t1507 + t1521 + t1547 + t1549 - t1553, t1402);
    let t1599 = piecewise3(t386, t1402 * t33 / 2.0 + t259 * t1497 / 2.0, -t481 * t1289 / 2.0 + t1594 * t57 / 2.0);
    (t1594, t1599)
}
