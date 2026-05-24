//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 745/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk745<F: Float>(t4939: F, t904: F, t2621: F, t4923: F, t1437: F, t1449: F, t2550: F, t2575: F, t2594: F, t2619: F, t305: F, t3822: F, t3860: F, t4840: F, t4842: F, t4846: F, t4878: F, t4881: F, t4886: F, t4892: F, t4908: F, t4911: F, t4919: F, t4924: F, t877: F, t896: F) -> (F, F, F) {
    let t4940 = t4939 * t904;
    let t4943 = t4923 * t2621;
    let t4946 = -F::new(0.310907e-1) * t4886 * t305 + F::new(2.0) * t3822 * t1437 - F::new(2.0) * t2550 * t4892 + F::new(1.0) * t877 * t4908 + F::cast_from(0.32163958997385070134e2_f64) * t2575 * t4911 + t4840 - t4842 + t4846 - t4878 - t4881 - F::cast_from(0.19751673498613801407e-1_f64) * t4919 + F::cast_from(0.11696447245269292414e1_f64) * t3860 * t1449 - F::cast_from(0.11696447245269292414e1_f64) * t2594 * t4924 + F::cast_from(0.5848223622634646207e0_f64) * t896 * t4940 + F::cast_from(0.17315859105681463759e2_f64) * t2619 * t4943;
    (t4940, t4943, t4946)
}
