//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1127/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1127<F: Float>(t3537: F, t94: F, t6076: F, t619: F, t77: F, t1317: F, t1679: F, t1290: F, t1981: F, t10289: F, t38: F, t3482: F, t76: F, t1313: F, t3418: F, t582: F) -> (F, F, F, F, F, F, F, F) {
    let t19308 = t94 * t3537;
    let t19342 = t77 * t6076 * t619;
    let t19345 = t1679 * t1317;
    let t19349 = t1981 * t1290;
    let t19352 = t10289 * t38;
    let t19380 = t76 * t3482;
    let t19388 = t77 * t1313 * t619;
    let t19396 = t3418 * t582;
    (t19308, t19342, t19345, t19349, t19352, t19380, t19388, t19396)
}
