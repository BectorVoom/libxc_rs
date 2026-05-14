//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1196/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1196<F: Float>(t226: F, t36098: F, t1379: F, t2407: F, t10818: F, t1705: F, t935: F, t18005: F, t6134: F, t2162: F, t64007: F, t3665: F, t818: F, t2425: F, t19733: F, t5570: F) -> (F, F, F, F, F, F, F, F) {
    let t64039 = t36098 * t226;
    let t64042 = t1379 * t2407;
    let t64050 = t1705 * t10818 * t935;
    let t64060 = t6134 * t18005;
    let t64063 = t64007 * t2162;
    let t64118 = t3665 * t818;
    let t64122 = t1379 * t2425;
    let t64135 = t19733 * t5570;
    (t64039, t64042, t64050, t64060, t64063, t64118, t64122, t64135)
}
