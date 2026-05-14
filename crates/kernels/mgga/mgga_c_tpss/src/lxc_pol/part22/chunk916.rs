//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 916/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk916<F: Float>(t45: F, t8101: F, t3645: F, t725: F, t1352: F, t2332: F, t8107: F, t8118: F, t8121: F, t10497: F, t150: F, t190: F, t2109: F, t3572: F, t3431: F, t80: F, t10353: F, t1310: F, t1985: F, t1992: F, t3595: F, t581: F, t741: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t10518 = 8.0 * t8101;
    let t10520 = 2.0 * t3645 * t725;
    let t10521 = t1352 * t2332;
    let t10522 = 4.0 * t8107;
    let t10523 = 0.4883052614935078681e-3 * t8118;
    let t10524 = 0.18311447306006545054e-3 * t8121;
    let t10525 = t150 * t10497;
    let t10526 = t10525 * t190;
    let t10528 = 4.0 * t3572 * t2109;
    let t10531 = t80 * t3431;
    let t10539 = piecewise3(t151, 0.0, 8.0 / 27.0 * t1310 * t1985 - 4.0 / 9.0 * t10531 * t581 - 2.0 / 9.0 * t3595 * t1992 + 2.0 / 3.0 * t741 * t10353);
    (t10518, t10520, t10521, t10522, t10523, t10524, t10526, t10528, t10539)
}
