//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1120/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1120<F: Float>(t15030: F, t15785: F, t1241: F, t1251: F, t5088: F, t3598: F, t1760: F, t3599: F, t11606: F, t225: F, t4941: F, t1751: F, t3481: F, t3630: F, t1238: F, t1252: F, t14972: F, t14980: F, t3487: F, t3593: F, t3600: F, t3631: F, t498: F, t5055: F, t5060: F, t5089: F) -> (F,) {
    let t15786 = t15030 + t15785;
    let t15787 = t1241 * t15786;
    let t15789 = t5088 * t1251;
    let t15790 = t3598 * t15789;
    let t15793 = t1760 * t3599;
    let t15794 = t11606 * t15793;
    let t15797 = t4941 * t225;
    let t15800 = t3481 * t1751;
    let t15802 = t1760 * t3630;
    let t15803 = t3598 * t15802;
    let t15806 = -t1238 * t15787 + 4.0 * t1238 * t15790 - 6.0 * t1238 * t15794 + 2.0 * t1238 * t15803 - 2.0 * t1252 * t14972 - 2.0 * t1252 * t14980 - 2.0 * t1252 * t15797 + t15800 * t498 + 4.0 * t3487 * t5060 - 2.0 * t3593 * t5089 + 2.0 * t3600 * t5055 - t3631 * t5055;
    (t15806,)
}
