//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1007/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1007<F: Float>(t18245: F, t423: F, t14858: F, t1703: F, t4869: F, t4879: F, t1117: F, t6021: F, t3264: F, t3315: F, t6020: F, t3313: F, t4781: F, t4785: F, t11277: F, t5988: F) -> (F, F, F, F, F, F, F) {
    let t18247 = 0.621814e-1 * t18245 * t423;
    let t18249 = 0.11696447245269292414e1 * t14858 * t1703;
    let t18251 = 0.11696447245269292414e1 * t4869 * t4879;
    let t18255 = t6021 * t1117;
    let t18257 = 2.0 * t3264 * t18255;
    let t18258 = t6020 * t3315;
    let t18259 = t18258 * t1117;
    let t18261 = 0.16081979498692535067e2 * t3313 * t18259;
    let t18262 = t4785 * t4781;
    let t18264 = 0.32163958997385070134e2 * t3313 * t18262;
    let t18265 = t5988 * t11277;
    (t18247, t18249, t18251, t18257, t18261, t18264, t18265)
}
