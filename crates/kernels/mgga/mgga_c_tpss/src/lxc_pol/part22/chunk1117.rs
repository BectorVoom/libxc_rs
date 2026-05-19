//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1117/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1117<F: Float>(t1542: F, t9347: F, t3002: F, t1089: F, t2994: F, t4197: F, t3009: F, t4207: F, t3021: F, t4192: F, t1551: F, t9589: F) -> (F, F, F, F, F) {
    let t12334 = t9347 * t1542;
    let t12335 = t12334 * t3002;
    let t12337 = F::cast_from(0.10389515463408878255e3_f64) * t1089 * t12335;
    let t12338 = t4197 * t2994;
    let t12340 = F::cast_from(0.11696447245269292414e1_f64) * t1089 * t12338;
    let t12342 = F::cast_from(0.34631718211362927518e2_f64) * t3009 * t4207;
    let t12344 = F::cast_from(0.17315859105681463759e2_f64) * t4192 * t3021;
    let t12346 = F::cast_from(0.5848223622634646207e0_f64) * t9589 * t1551;
    (t12337, t12340, t12342, t12344, t12346)
}
