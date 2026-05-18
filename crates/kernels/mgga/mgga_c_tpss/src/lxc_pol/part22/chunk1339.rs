//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1339/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1339<F: Float>(t41371: F, t520: F, t1656: F, t3326: F, t1232: F, t1265: F, t3260: F, t4460: F, t18495: F, t6259: F, t3259: F, t41590: F) -> (F, F, F, F, F, F) {
    let t65738 = t41371 * t520;
    let t65783 = t1656 * t3326 * t520;
    let t65818 = t3260 * t1265 * t1232;
    let t65867 = t4460 * t1265;
    let t65871 = t6259 * t18495;
    let t65878 = t41590 * t3259;
    (t65738, t65783, t65818, t65867, t65871, t65878)
}
