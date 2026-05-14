//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 417/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk417<F: Float>(t2936: F, t300: F, t2898: F, t938: F, t961: F, t2904: F, t2906: F, t951: F, t959: F, t2924: F, t942: F, t2929: F, t2932: F, t2262: F, t338: F, t964: F, t969: F) -> (F, F, F, F, F, F, F, F) {
    let t2937 = t300 * t2936;
    let t2939 = 0.19751673498613801407e-1 * t300 * t2898;
    let t2940 = t300 * t938;
    let t2942 = 0.11696447245269292414e1 * t2940 * t961;
    let t2944 = t2904 * t2906 * t951;
    let t2946 = 0.11696447245269292414e1 * t959 * t2944;
    let t2948 = t942 * t2924 * t951;
    let t2950 = 0.5848223622634646207e0 * t959 * t2948;
    let t2951 = t2929 * t2906;
    let t2952 = t2951 * t2932;
    let t2954 = 0.17315859105681463759e2 * t959 * t2952;
    let t2955 = t2262 * t338;
    let t2958 = t964 * t969;
    (t2937, t2939, t2942, t2946, t2950, t2954, t2955, t2958)
}
