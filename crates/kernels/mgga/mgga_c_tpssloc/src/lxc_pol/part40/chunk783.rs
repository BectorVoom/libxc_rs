//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 783/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk783<F: Float>(t300: F, t6091: F, t6064: F, t1703: F, t4869: F, t1156: F, t3375: F, t6068: F, t1164: F, t1147: F, t6084: F, t3400: F, t3403: F, t338: F, t5416: F, t3441: F, t5392: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6092 = t300 * t6091;
    let t6094 = 0.19751673498613801407e-1 * t300 * t6064;
    let t6096 = 0.11696447245269292414e1 * t4869 * t1703;
    let t6098 = t3375 * t6068 * t1156;
    let t6100 = 0.11696447245269292414e1 * t1164 * t6098;
    let t6102 = t1147 * t6084 * t1156;
    let t6104 = 0.5848223622634646207e0 * t1164 * t6102;
    let t6105 = t3400 * t6068;
    let t6106 = t6105 * t3403;
    let t6108 = 0.17315859105681463759e2 * t1164 * t6106;
    let t6109 = t5416 * t338;
    let t6119 = t3441 * t5392;
    (t6092, t6094, t6096, t6098, t6100, t6102, t6104, t6105, t6106, t6108, t6109, t6119)
}
