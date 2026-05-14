//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 266/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk266<F: Float>(t28: F, t1409: F, t1534: F, t1649: F, t1768: F, t265: F, t506: F, t52: F, t1647: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t1773 = piecewise3(t401, t1534 * t28 / 2.0 + t265 * t1649 / 2.0, -t506 * t1409 / 2.0 + t1768 * t52 / 2.0);
    let t1774 = t1647 + t1773;
    (t1774,)
}
