//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 175/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk175<F: Float>(t141: F, t6: F, t36: F, t951: F, t214: F, t243: F, t242: F, t7: F, t5: F, t12: F, t140: F, t368: F, t142: F, t410: F, t417: F, t431: F) -> (F, F, F, F, F, F, F, F, F) {
    let t952 = t141 * t6;
    let t953 = t952 * t36;
    let t954 = t951 * t953;
    let t956 = t243 * t214;
    let t957 = t242 * t956;
    let t959 = t7 * t214;
    let t960 = t5 * t959;
    let t962 = 1.0/f64::sqrt(t12);
    let t963 = t962 * t140;
    let t964 = t963 * t953;
    let t966 = t368 * t956;
    let t969 = t142 * t6 * t36;
    let t971 = -0.57538888888888888889e0 * t954 + 0.11507777777777777778e1 * t957 + 0.40256666666666666667e0 * t960 + 0.366775e-1 * t964 + 0.73355e-1 * t966 + 0.137975e0 * t969;
    let t973 = t410 * t971 * t417;
    let t975 = 0.5848223622634646207e0 * t431 * t973;
    (t954, t957, t959, t960, t964, t966, t969, t971, t975)
}
