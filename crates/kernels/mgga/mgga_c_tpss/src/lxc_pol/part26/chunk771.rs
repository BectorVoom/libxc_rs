//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 771/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk771<F: Float>(t294: F, t5184: F, t5157: F, t1551: F, t4192: F, t1081: F, t2973: F, t5161: F, t1089: F, t1072: F, t5177: F, t2998: F, t3001: F, t1101: F, t4579: F, t926: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5185 = t294 * t5184;
    let t5187 = 0.19751673498613801407e-1 * t294 * t5157;
    let t5189 = 0.11696447245269292414e1 * t4192 * t1551;
    let t5191 = t2973 * t5161 * t1081;
    let t5193 = 0.11696447245269292414e1 * t1089 * t5191;
    let t5195 = t1072 * t5177 * t1081;
    let t5197 = 0.5848223622634646207e0 * t1089 * t5195;
    let t5198 = t2998 * t5161;
    let t5199 = t5198 * t3001;
    let t5201 = 0.17315859105681463759e2 * t1089 * t5199;
    let t5206 = t1101 * t4579;
    let t5207 = t926 * t5206;
    (t5185, t5187, t5189, t5191, t5193, t5195, t5197, t5198, t5199, t5201, t5206, t5207)
}
