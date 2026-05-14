//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 781/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk781<F: Float>(t1000: F, t1020: F, t1025: F, t10263: F, t1041: F, t1046: F, t10517: F, t10860: F, t10863: F, t10866: F, t10871: F, t10873: F, t10876: F, t10879: F, t10883: F, t10886: F, t10891: F, t10896: F, t10898: F, t10904: F, t10909: F, t10915: F, t10919: F, t10923: F, t10927: F, t3043: F, t3057: F, t3109: F, t3117: F, t3123: F, t3134: F) -> (F,) {
    let t10929 = 19.0 / 576.0 * t10517 * t1025 + t1020 * t10860 / 3072.0 - t10863 * t1046 / 144.0 + t10866 / 1152.0 - t10871 / 6912.0 - t10873 / 216.0 - t10876 * t10879 / 512.0 + t10883 * t10886 / 3072.0 + t10891 * t3043 / 192.0 - t10896 / 1536.0 - t10898 * t1025 / 96.0 - t3109 * t3123 / 192.0 - t10904 * t3134 / 96.0 + t10909 / 1536.0 + t3117 * t3057 / 1536.0 - t1041 * t10915 / 768.0 + 5.0 / 4608.0 * t1041 * t10919 - t10923 / 432.0 + 11.0 / 108.0 * t10263 * t1000 - t10927 / 54.0;
    (t10929,)
}
