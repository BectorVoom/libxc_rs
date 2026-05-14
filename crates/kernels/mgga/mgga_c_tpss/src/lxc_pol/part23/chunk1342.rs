//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1342/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1342<F: Float>(t33: F, t259: F, t479: F, t64856: F, t68628: F, t10353: F, t1289: F, t1893: F, t19179: F, t1992: F, t20936: F, t3431: F, t57: F, t581: F, t6048: F, t65036: F, t6534: F, t118: F, t1663: F, t19272: F, t20957: F, t2106: F, t3166: F, t3396: F, t544: F, t6480: F, t65063: F, t65066: F, t65069: F, t65071: F, t65079: F, t65088: F, t65091: F, t65093: F, t65096: F, t65099: F, t65101: F, t65106: F, t65109: F, t6544: F, t65975: F, t68173: F, t68189: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t68629 = piecewise3(t480, t68628, t64856);
    let t68641 = piecewise3(t386, t65036, t68629 * t57 / 2.0 - t20936 * t581 - t6534 * t1992 / 2.0 - t19179 * t1289 / 2.0 - t6048 * t3431 - t1893 * t10353 / 2.0);
    let t68645 = t6544 * t3396 - t65063 - t65066 - t65069 - t65071 + t65079 - 2.0 * t20957 * t2106 + t65088 - t65091 + (t65975 + t68173) * t544 - t65093 - t65096 - t65099 - t65101 + t19272 * t1663 - t118 * (t68189 + t68641) - t6480 * t3166 - t65106 + t65109;
    (t68645,)
}
