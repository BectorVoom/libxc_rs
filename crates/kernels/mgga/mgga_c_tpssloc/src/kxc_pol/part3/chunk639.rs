//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 639/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk639<F: Float>(t3040: F, t381: F, t1932: F, t3131: F, t1022: F, t1049: F, t1060: F, t3120: F, t1014: F, t3032: F, t3031: F) -> (F, F, F, F, F, F, F) {
    let t3187 = t381 * t3040;
    let t3188 = t1932 * t3131;
    let t3189 = t3187 * t3188;
    let t3192 = t1049 * t1022;
    let t3193 = t3192 * t1060;
    let t3196 = t381 * t3120;
    let t3197 = t3196 * t1060;
    let t3199 = t3032 * t1014;
    let t3200 = t3031 * t3199;
    (t3187, t3188, t3189, t3193, t3197, t3199, t3200)
}
