//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 994/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk994<F: Float>(t1694: F, t3403: F, t1155: F, t1129: F, t1138: F, t1148: F, t1157: F, t1683: F, t1695: F, t3327: F, t3332: F, t3357: F, t3371: F, t3376: F, t3401: F, t436: F, t4739: F, t4742: F, t4744: F, t4747: F, t4784: F, t4788: F, t4794: F, t4797: F, t4802: F, t4820: F, t4824: F, t4833: F, t4835: F, t4840: F, t4858: F) -> (F, F, F) {
    let t4861 = t1694 * t3403;
    let t4862 = t4861 * t1155;
    let t4865 = -F::cast_from(0.310907e-1_f64) * t4794 * t436 + F::cast_from(1.0_f64) * t4797 * t1138 + F::cast_from(1.0_f64) * t3327 * t1683 - F::cast_from(2.0_f64) * t3332 * t4802 + F::cast_from(1.0_f64) * t1129 * t4820 + F::cast_from(0.32163958997385070134e2_f64) * t3357 * t4824 + t4739 - t4742 - t4744 + t4747 - t4784 - t4788 - F::cast_from(0.19751673498613801407e-1_f64) * t4833 + F::cast_from(0.5848223622634646207e0_f64) * t4835 * t1157 + F::cast_from(0.5848223622634646207e0_f64) * t3371 * t1695 - F::cast_from(0.11696447245269292414e1_f64) * t3376 * t4840 + F::cast_from(0.5848223622634646207e0_f64) * t1148 * t4858 + F::cast_from(0.17315859105681463759e2_f64) * t3401 * t4862;
    (t4861, t4862, t4865)
}
