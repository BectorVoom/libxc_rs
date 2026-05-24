//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1140/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1140<F: Float>(t1062: F, t15692: F, t1072: F, t5156: F, t1081: F, t15351: F, t1053: F, t5124: F, t1054: F, t1063: F, t1073: F, t1082: F, t12070: F, t1543: F, t15607: F, t15609: F, t2925: F, t2969: F, t4158: F, t4181: F, t5146: F, t5149: F, t5162: F, t5178: F, t5181: F, t9359: F, t9370: F, t9419: F) -> F {
    let t15693 = t15692 * t1062;
    let t15698 = t5156 * t1072;
    let t15709 = t15351 * t1081;
    let t15714 = t5124 * t1053;
    let t15717 = F::new(1.0) * t2925 * t5146 + F::new(1.0) * t1054 * t15693 + F::cast_from(0.32163958997385070134e2_f64) * t9419 * t5149 + F::cast_from(0.5848223622634646207e0_f64) * t15698 * t1082 + F::cast_from(0.11696447245269292414e1_f64) * t12070 * t1543 + F::cast_from(0.11696447245269292414e1_f64) * t4158 * t4181 - F::cast_from(0.11696447245269292414e1_f64) * t9359 * t5162 + F::cast_from(0.5848223622634646207e0_f64) * t2969 * t5178 + F::cast_from(0.5848223622634646207e0_f64) * t1073 * t15709 + F::cast_from(0.17315859105681463759e2_f64) * t9370 * t5181 + F::new(1.0) * t15714 * t1063 + t15607 - t15609;
    t15717
}
