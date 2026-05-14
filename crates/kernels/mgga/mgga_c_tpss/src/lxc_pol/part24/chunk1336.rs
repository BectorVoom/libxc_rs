//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1336/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1336<F: Float>(t33: F, t259: F, t479: F, t70835: F, t70885: F, t70920: F, t70963: F, t70784: F, t1289: F, t13335: F, t1749: F, t20070: F, t21524: F, t3431: F, t4579: F, t5686: F, t57: F, t581: F, t6222: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t70965 = t70835 + t70885 + t70920 + t70963;
    let t70966 = piecewise3(t480, 0.0, t70784);
    let t70978 = piecewise3(t386, t70965, t70966 * t57 / 2.0 - t21524 * t581 / 2.0 - t20070 * t1289 - t6222 * t3431 - t5686 * t4579 / 2.0 - t1749 * t13335 / 2.0);
    (t70978,)
}
