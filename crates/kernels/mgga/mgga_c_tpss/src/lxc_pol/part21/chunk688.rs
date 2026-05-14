//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 688/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk688<F: Float>(t1268: F, t1270: F, t1190: F, t2222: F, t1183: F, t72: F, t732: F, t1193: F, t2345: F, t2215: F, t724: F, t489: F, t1173: F, t1184: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3184 = t1268 * t1270;
    let t3189 = 0.24415263074675393405e-3 * t1190 * t2222;
    let t3190 = t1183 * t72;
    let t3191 = t3190 * t732;
    let t3192 = 0.36622894612013090108e-3 * t3191;
    let t3194 = 0.11696447245269292414e1 * t1193 * t2345;
    let t3196 = 0.17315859105681463759e2 * t1193 * t2215;
    let t3197 = t1183 * t724;
    let t3198 = t489 * t3197;
    let t3199 = 2.0 * t3198;
    let t3200 = t1173 * t1184;
    let t3201 = 8.0 * t3200;
    let t3202 = t1268 * t1268;
    (t3184, t3189, t3190, t3191, t3192, t3194, t3196, t3197, t3198, t3199, t3200, t3201, t3202)
}
