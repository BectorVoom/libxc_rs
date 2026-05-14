//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1343/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1343<F: Float>(t28: F, t265: F, t504: F, t76559: F, t78240: F, t78305: F, t78342: F, t79538: F, t1409: F, t1534: F, t1649: F, t1768: F, t20217: F, t20390: F, t21076: F, t22414: F, t506: F, t52: F, t5398: F, t5669: F, t5966: F, t6279: F, t75912: F, t77953: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t79541 = piecewise3(t505, t78240 + t78305 + t78342 + t79538, t76559);
    let t79553 = piecewise3(t401, t76559 * t28 / 2.0 + 2.0 * t21076 * t1649 + 3.0 * t5669 * t5966 + 2.0 * t1534 * t20390 + t265 * t77953 / 2.0, t79541 * t52 / 2.0 - 2.0 * t22414 * t1409 - 3.0 * t6279 * t5398 - 2.0 * t1768 * t20217 - t506 * t75912 / 2.0);
    (t79553,)
}
