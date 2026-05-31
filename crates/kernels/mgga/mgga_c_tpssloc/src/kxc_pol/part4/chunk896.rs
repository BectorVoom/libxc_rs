//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 896/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk896<F: Float>(t1058: F, t10936: F, t3030: F, t990: F, t3032: F, t3129: F, t3038: F, t2775: F, t283: F, t3185: F, t3199: F, t1014: F, t10471: F) -> (F, F, F, F, F, F, F) {
    let t10937 = t1058 * t10936;
    let t10947 = t990 * t3030;
    let t10948 = t10947 * t3032;
    let t10949 = t10948 * t3129;
    let t10952 = t10948 * t3038;
    let t10969 = F::cast_from(1.0_f64) / t283 / t2775;
    let t11034 = t10947 * t3185;
    let t11037 = t10947 * t3199;
    let t11045 = t10471 * t1014;
    (t10937, t10949, t10952, t10969, t11034, t11037, t11045)
}
