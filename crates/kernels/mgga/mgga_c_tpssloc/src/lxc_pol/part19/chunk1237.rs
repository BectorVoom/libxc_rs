//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1237/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1237<F: Float>(t42790: F, t42824: F, t42860: F, t42899: F, t42933: F, t42966: F, t43034: F, t43079: F, t225: F, t10427: F, t13969: F, t3130: F, t10432: F, t3039: F, t1021: F, t1025: F, t1041: F, t1044: F, t1046: F, t10863: F, t248: F, t3043: F, t3064: F, t3131: F, t369: F, t378: F, t41671: F, t42422: F, t42729: F, t42731: F, t42735: F, t42743: F, t42746: F, t42752: F, t42756: F, t68: F) -> (F, F, F) {
    let t43082 = t42790 + t42824 + t42860 + t42899 + t42933 + t42966 + t43034 + t43079;
    let t43083 = t43082 * t225;
    let t43094 = t3130 * t13969 * t10427;
    let t43097 = t3039 * t13969 * t10432;
    let t43099 = -5.0 / 216.0 * t10863 * t3064 + t42729 / 576.0 + t42731 / 72.0 + t42735 / 2304.0 + t1041 * t248 * t1044 * t41671 / 4608.0 - t42743 * t3043 / 512.0 + t42746 * t1046 / 1152.0 + t42752 / 3888.0 + t42756 * t1025 / 768.0 + t43083 * t68 * t369 * t378 / 3072.0 + t3130 * t248 * t1021 * t42422 * t3131 / 512.0 + t43094 / 192.0 - t43097 / 384.0;
    (t43082, t43083, t43099)
}
