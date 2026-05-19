//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 644/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk644<F: Float>(t1156: F, t6084: F, t3403: F, t6068: F, t1129: F, t1148: F, t1683: F, t1695: F, t3332: F, t3357: F, t3376: F, t3401: F, t436: F, t4797: F, t4835: F, t5985: F, t5987: F, t5991: F, t6023: F, t6026: F, t6031: F, t6037: F, t6053: F, t6056: F, t6064: F, t6069: F) -> (F, F, F) {
    let t6085 = t6084 * t1156;
    let t6088 = t6068 * t3403;
    let t6091 = -F::new(0.310907e-1) * t6031 * t436 + F::new(2.0) * t4797 * t1683 - F::new(2.0) * t3332 * t6037 + F::new(1.0) * t1129 * t6053 + F::cast_from(0.32163958997385070134e2_f64) * t3357 * t6056 + t5985 - t5987 + t5991 - t6023 - t6026 - F::cast_from(0.19751673498613801407e-1_f64) * t6064 + F::cast_from(0.11696447245269292414e1_f64) * t4835 * t1695 - F::cast_from(0.11696447245269292414e1_f64) * t3376 * t6069 + F::cast_from(0.5848223622634646207e0_f64) * t1148 * t6085 + F::cast_from(0.17315859105681463759e2_f64) * t3401 * t6088;
    (t6085, t6088, t6091)
}
