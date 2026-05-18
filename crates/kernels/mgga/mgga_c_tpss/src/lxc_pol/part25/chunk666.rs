//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 666/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk666<F: Float>(t1091: F, t4192: F, t1551: F, t3009: F, t1542: F, t2973: F, t1082: F, t1089: F, t1072: F, t1081: F, t4180: F, t2998: F) -> (F, F, F, F, F, F, F, F) {
    let t4194 = F::new(0.5848223622634646207e0) * t4192 * t1091;
    let t4196 = F::new(0.5848223622634646207e0) * t3009 * t1551;
    let t4197 = t2973 * t1542;
    let t4198 = t4197 * t1082;
    let t4200 = F::new(0.11696447245269292414e1) * t1089 * t4198;
    let t4202 = t1072 * t4180 * t1081;
    let t4204 = F::new(0.5848223622634646207e0) * t1089 * t4202;
    let t4205 = t2998 * t1542;
    (t4194, t4196, t4197, t4198, t4200, t4202, t4204, t4205)
}
