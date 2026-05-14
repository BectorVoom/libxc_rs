//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 994/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk994<F: Float>(t12255: F, t1343: F, t820: F, t3777: F, t3798: F, t1354: F, t1307: F, t3719: F, t3870: F, t12189: F, t1329: F, t3726: F, t3770: F, t119: F, t12012: F, t210: F) -> (F, F, F, F, F, F, F) {
    let t12297 = t1343 * t820 * t12255;
    let t12300 = t3777 * t3798;
    let t12301 = t12300 * t1354;
    let t12303 = t1307 * t3719;
    let t12305 = t3870 * t820 * t12303;
    let t12308 = t12189 * t1329;
    let t12310 = t3726 * t3770;
    let t12313 = t210 * t119 * t12012;
    (t12297, t12301, t12303, t12305, t12308, t12310, t12313)
}
