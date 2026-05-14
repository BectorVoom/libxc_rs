//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1318/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1318<F: Float>(t33: F, t259: F, t479: F, t64909: F, t64944: F, t64996: F, t65034: F, t64856: F, t10353: F, t1289: F, t1749: F, t18279: F, t1992: F, t20070: F, t3431: F, t5686: F, t57: F, t581: F, t6222: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t65036 = t64909 + t64944 + t64996 + t65034;
    let t65037 = piecewise3(t480, 0.0, t64856);
    let t65049 = piecewise3(t386, t65036, t65037 * t57 / 2.0 - t20070 * t581 - t6222 * t1992 / 2.0 - t18279 * t1289 / 2.0 - t5686 * t3431 - t1749 * t10353 / 2.0);
    (t65049,)
}
