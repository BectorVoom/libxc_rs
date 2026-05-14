//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1108/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1108<F: Float>(t15548: F, t3515: F, t486: F, t5011: F, t4978: F, t4582: F, t11709: F, t11738: F, t11814: F, t11825: F, t1213: F, t1227: F, t15524: F, t15527: F, t15531: F, t15535: F, t15541: F, t15545: F, t1737: F, t1748: F, t3490: F, t3506: F, t3531: F, t3536: F, t4980: F, t4989: F, t5014: F, t5024: F) -> (F, F) {
    let t15550 = t3515 * t15548 / 2304.0;
    let t15553 = t486 * t5011;
    let t15554 = t15553 * t4978;
    let t15555 = t4582 * t15554;
    let t15558 = t5024 * t3531 / 432.0 - t11825 * t1748 / 4608.0 + t11814 * t1737 / 3072.0 + t3536 * t5014 / 1536.0 + t15524 + t1213 * t15527 / 3072.0 - t3515 * t15531 / 3072.0 + t11738 * t15535 / 3072.0 + 5.0 / 6912.0 * t3490 * t4989 + 5.0 / 6912.0 * t1227 * t15541 + 5.0 / 13824.0 * t1227 * t15545 - t15550 + t11709 * t4980 / 768.0 + t3506 * t15555 / 768.0;
    (t15553, t15558)
}
