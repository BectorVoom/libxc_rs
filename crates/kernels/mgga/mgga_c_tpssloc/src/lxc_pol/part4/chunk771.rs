//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 771/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk771<F: Float>(t28: F, t265: F, t504: F, t5669: F, t6278: F, t1409: F, t1534: F, t1649: F, t1768: F, t506: F, t52: F, t5398: F, t5966: F, t5962: F, t1268: F, t1458: F, t4028: F, t5450: F, t5456: F, t5493: F, t88: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t6279 = piecewise3(t505, t6278, t5669);
    let t6286 = piecewise3(t401, t5669 * t28 / 2.0 + t1534 * t1649 + t265 * t5966 / 2.0, t6279 * t52 / 2.0 - t1768 * t1409 - t506 * t5398 / 2.0);
    let t6287 = t5962 + t6286;
    let t6295 = 2.0 * t1268 * t5493 + 4.0 * t1458 * t4028 + 2.0 * t5456 * t88 + t5450;
    (t6279, t6287, t6295)
}
