//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1062/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1062<F: Float>(t125: F, t4397: F, t1233: F, t3273: F, t3327: F, t4471: F, t10151: F, t4416: F, t12863: F, t4415: F, t3240: F, t4409: F, t10078: F, t10082: F, t10100: F, t10104: F, t10118: F, t10131: F, t10138: F, t3271: F) -> (F, F, F, F, F, F) {
    let t12968 = t125 * t4397;
    let t12970 = t3273 * t12968 * t1233;
    let t12974 = t3273 * t4471 * t3327;
    let t12978 = t3273 * t4416 * t10151;
    let t12982 = t4415 * t12863 * t1233;
    let t12986 = t4415 * t4416 * t3327;
    let t12993 = 7.0 / 72.0 * t3240 * t4409;
    let t12994 = -119.0 / 6912.0 * t10078 - 7.0 / 2304.0 * t10082 + 7.0 / 4608.0 * t10100 + t3271 * t12970 / 384.0 + t3271 * t12974 / 768.0 + t3271 * t12978 / 768.0 - t3271 * t12982 / 1536.0 - t3271 * t12986 / 3072.0 - t10104 - 7.0 / 576.0 * t10118 + 7.0 / 144.0 * t10131 - 7.0 / 48.0 * t10138 + t12993;
    (t12970, t12974, t12978, t12982, t12986, t12994)
}
