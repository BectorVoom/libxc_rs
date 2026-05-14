//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1182/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1182<F: Float>(t17974: F, t3685: F, t19695: F, t19697: F, t5543: F, t136: F, t1693: F, t799: F, t61087: F, t18005: F, t6134: F, t19733: F, t5570: F, t30: F, t31814: F, t2: F, t2436: F) -> (F, F, F, F, F, F, F, F) {
    let t63977 = t17974 * t3685;
    let t63990 = t5543 * t19695 * t19697;
    let t63993 = t1693 * t799 * t136;
    let t63998 = 119.0 / 864.0 * t61087;
    let t64060 = t6134 * t18005;
    let t64135 = t19733 * t5570;
    let t64247 = t31814 * t30;
    let t64300 = t2436 * t2;
    (t63977, t63990, t63993, t63998, t64060, t64135, t64247, t64300)
}
