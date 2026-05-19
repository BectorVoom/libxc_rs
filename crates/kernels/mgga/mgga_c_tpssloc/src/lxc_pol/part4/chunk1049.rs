//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1049/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1049<F: Float>(t17349: F, t932: F, t5769: F, t942: F, t17297: F, t951: F, t13515: F, t1557: F, t4354: F, t4396: F, t10747: F, t10765: F, t10825: F, t14332: F, t1581: F, t17197: F, t2900: F, t4449: F, t4472: F, t5762: F, t5775: F, t5791: F, t5794: F, t924: F, t943: F, t952: F) -> (F, F, F) {
    let t17350 = t17349 * t932;
    let t17355 = t5769 * t942;
    let t17366 = t17297 * t951;
    let t17372 = F::new(2.0) * t13515 * t1557;
    let t17374 = F::new(2.0) * t4354 * t4396;
    let t17375 = -t17197 + F::new(1.0) * t924 * t17350 + F::cast_from(0.32163958997385070134e2_f64) * t10765 * t5762 + F::cast_from(0.5848223622634646207e0_f64) * t17355 * t952 + F::cast_from(0.11696447245269292414e1_f64) * t14332 * t1581 + F::cast_from(0.11696447245269292414e1_f64) * t4449 * t4472 - F::cast_from(0.11696447245269292414e1_f64) * t10747 * t5775 + F::cast_from(0.5848223622634646207e0_f64) * t2900 * t5791 + F::cast_from(0.5848223622634646207e0_f64) * t943 * t17366 + F::cast_from(0.17315859105681463759e2_f64) * t10825 * t5794 - t17372 - t17374;
    (t17372, t17374, t17375)
}
