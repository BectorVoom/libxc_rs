//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2565/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2565<F: Float>(t11185: F, t21724: F, t11297: F, t11365: F, t1138: F, t11415: F, t1155: F, t1157: F, t15146: F, t1695: F, t18637: F, t18644: F, t18785: F, t21836: F, t21947: F, t21952: F, t3376: F, t3401: F, t4857: F, t4858: F, t51427: F, t51730: F, t6037: F, t6069: F, t6084: F, t71850: F, t71853: F, t71855: F, t71860: F, t71863: F) -> (F, F) {
    let t71867 = F::new(6.0) * t11185 * t21724;
    let t71868 = F::cast_from(0.96491876992155210402e2_f64) * t15146 * t18644 - F::cast_from(0.57895126195293126241e3_f64) * t51427 * t18637 - F::cast_from(0.14035736694323150897e2_f64) * t11365 * t21947 * t1155 + F::cast_from(0.10526802520742363173e2_f64) * t3401 * t6069 * t4857 - F::cast_from(0.35089341735807877242e1_f64) * t11297 * t21836 - F::cast_from(0.35089341735807877242e1_f64) * t3376 * t4858 * t6084 - F::cast_from(0.35089341735807877242e1_f64) * t3376 * t1695 * t18785 + t71850 - t71853 + t71855 - F::new(6.0) * t51730 * t6037 + F::new(6.0) * t11415 * t21952 + F::cast_from(0.5848223622634646207e0_f64) * t71860 * t1157 + F::new(1.0) * t71863 * t1138 - t71867;
    (t71867, t71868)
}
