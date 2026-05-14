//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1161/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1161<F: Float>(t1232: F, t1265: F, t520: F, t18497: F, t5731: F, t5740: F, t1768: F, t3384: F, t1258: F, t3255: F, t3259: F, t3260: F, t5745: F, t3326: F, t1773: F, t18471: F, t522: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18499 = t1265 * t1232 * t520;
    let t18500 = t18497 * t18499;
    let t18504 = t5740 * t5731 * t1265;
    let t18508 = t5740 * t1768 * t3384;
    let t18511 = t1258 * t3255;
    let t18512 = t1768 * t3259;
    let t18513 = t18512 * t3260;
    let t18514 = t18511 * t18513;
    let t18518 = t5731 * t1232 * t520;
    let t18519 = t5745 * t18518;
    let t18523 = t1768 * t3326 * t520;
    let t18524 = t5745 * t18523;
    let t18526 = t18512 * t520;
    let t18527 = t5745 * t18526;
    let t18530 = t1773 * t522 * t18471;
    (t18499, t18500, t18504, t18508, t18511, t18514, t18519, t18524, t18527, t18530)
}
