//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1078/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1078<F: Float>(t1542: F, t9347: F, t3002: F, t1089: F, t2994: F, t4197: F, t3009: F, t4207: F, t3021: F, t4192: F, t1551: F, t9589: F, t1072: F, t1081: F, t12135: F, t12159: F, t12161: F, t12164: F, t12167: F, t12170: F, t12243: F, t12246: F, t12250: F, t12253: F, t12257: F, t12260: F, t12276: F) -> (F, F, F, F, F, F, F) {
    let t12334 = t9347 * t1542;
    let t12335 = t12334 * t3002;
    let t12337 = 0.10389515463408878255e3 * t1089 * t12335;
    let t12338 = t4197 * t2994;
    let t12340 = 0.11696447245269292414e1 * t1089 * t12338;
    let t12342 = 0.34631718211362927518e2 * t3009 * t4207;
    let t12344 = 0.17315859105681463759e2 * t4192 * t3021;
    let t12346 = 0.5848223622634646207e0 * t9589 * t1551;
    let t12348 = t1072 * t12135 * t1081;
    let t12350 = 0.5848223622634646207e0 * t1089 * t12348;
    let t12351 = t12276 - t12159 + t12161 - t12164 - t12167 - t12170 - t12243 - t12246 + t12250 + t12253 + t12257 + t12260 + t12337 + t12340 - t12342 - t12344 - t12346 - t12350;
    (t12337, t12340, t12342, t12344, t12346, t12350, t12351)
}
