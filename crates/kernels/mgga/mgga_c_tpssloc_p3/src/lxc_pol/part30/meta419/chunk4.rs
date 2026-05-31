//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1604/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1604<F: Float>(t1137: F, t18893: F, t1147: F, t6063: F, t1129: F, t11303: F, t11361: F, t1138: F, t11415: F, t1157: F, t15121: F, t15141: F, t1683: F, t1695: F, t18837: F, t18839: F, t18840: F, t3327: F, t4797: F, t4820: F, t4835: F, t4858: F, t6037: F, t6053: F, t6056: F, t6088: F) -> F {
    let t18894 = t18893 * t1137;
    let t18899 = t6063 * t1147;
    let t18906 = F::cast_from(0.17315859105681463759e2_f64) * t11361 * t6088 - t18837 - t18839 + F::cast_from(1.0_f64) * t18840 * t1138 + F::cast_from(2.0_f64) * t15141 * t1683 + F::cast_from(2.0_f64) * t4797 * t4820 - F::cast_from(2.0_f64) * t11303 * t6037 + F::cast_from(1.0_f64) * t3327 * t6053 + F::cast_from(1.0_f64) * t1129 * t18894 + F::cast_from(0.32163958997385070134e2_f64) * t11415 * t6056 + F::cast_from(0.5848223622634646207e0_f64) * t18899 * t1157 + F::cast_from(0.11696447245269292414e1_f64) * t15121 * t1695 + F::cast_from(0.11696447245269292414e1_f64) * t4835 * t4858;
    t18906
}
