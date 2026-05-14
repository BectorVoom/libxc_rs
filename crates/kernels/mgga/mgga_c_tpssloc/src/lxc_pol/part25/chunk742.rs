//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 742/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk742<F: Float>(t10266: F, t10357: F, t225: F, t68: F, t369: F, t10195: F, t2979: F, t1031: F, t3077: F, t1036: F, t3078: F, t1032: F, t3082: F, t2393: F, t374: F, t376: F) -> (F, F, F, F, F, F, F, F) {
    let t10358 = t10266 + t10357;
    let t10359 = t10358 * t225;
    let t10360 = t10359 * t68;
    let t10361 = t10360 * t369;
    let t10364 = t2979 * t10195;
    let t10367 = t3077 * t1031;
    let t10370 = t3078 * t1036;
    let t10372 = t1032 * t3082;
    let t10375 = t374 * t2393 * t376;
    (t10358, t10359, t10361, t10364, t10367, t10370, t10372, t10375)
}
