//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 801/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk801<F: Float>(t1009: F, t5848: F, t1011: F, t1019: F, t10422: F, t5908: F, t3070: F, t225: F, t5915: F, t1057: F, t5972: F, t690: F) -> (F, F, F, F, F) {
    let t18028 = t5848 * t1009;
    let t18029 = t18028 * t1011;
    let t18030 = t18029 * t1019;
    let t18041 = t10422 * t5908;
    let t18042 = t3070 * t18041;
    let t18074 = t5915 * t225;
    let t18086 = t18028 * t1057;
    let t18203 = t690 * t5972;
    (t18030, t18042, t18074, t18086, t18203)
}
