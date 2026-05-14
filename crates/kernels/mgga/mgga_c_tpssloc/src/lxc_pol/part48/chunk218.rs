//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 218/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk218<F: Float>(t950: F, t951: F, t300: F, t311: F, t890: F, t916: F, t919: F, t924: F, t933: F, t939: F, t943: F, t315: F, t942: F, t338: F, t615: F, t134: F, t340: F) -> (F, F, F, F, F, F, F, F) {
    let t952 = t950 * t951;
    let t956 = t300 * (-0.310907e-1 * t919 * t311 + 1.0 * t924 * t933 + t890 - t916 - 0.19751673498613801407e-1 * t939 + 0.5848223622634646207e0 * t943 * t952);
    let t958 = 0.19751673498613801407e-1 * t300 * t939;
    let t959 = t300 * t315;
    let t961 = t942 * t950 * t951;
    let t963 = 0.5848223622634646207e0 * t959 * t961;
    let t964 = t615 * t338;
    let t967 = t134 * t340;
    (t952, t956, t958, t959, t961, t963, t964, t967)
}
