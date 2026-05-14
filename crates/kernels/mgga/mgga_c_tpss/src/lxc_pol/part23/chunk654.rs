//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 654/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk654<F: Float>(t1091: F, t3009: F, t1081: F, t2973: F, t2975: F, t1089: F, t1072: F, t2993: F, t2998: F, t3001: F, t215: F, t442: F, t671: F, t441: F, t1102: F, t140: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3011 = 0.11696447245269292414e1 * t3009 * t1091;
    let t3013 = t2973 * t2975 * t1081;
    let t3015 = 0.11696447245269292414e1 * t1089 * t3013;
    let t3017 = t1072 * t2993 * t1081;
    let t3019 = 0.5848223622634646207e0 * t1089 * t3017;
    let t3020 = t2998 * t2975;
    let t3021 = t3020 * t3001;
    let t3023 = 0.17315859105681463759e2 * t1089 * t3021;
    let t3025 = t215 * t671 * t442;
    let t3027 = t441 * t3025 / 432.0;
    let t3028 = t140 * t1102;
    (t3011, t3013, t3015, t3017, t3019, t3021, t3023, t3025, t3027, t3028)
}
