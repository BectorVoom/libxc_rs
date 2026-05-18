//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 644/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk644<F: Float>(t1062: F, t2949: F, t1052: F, t412: F, t420: F, t2931: F, t2834: F, t2836: F, t2843: F, t2848: F, t2852: F, t434: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2950 = t2949 * t1062;
    let t2953 = t1052 * t1052;
    let t2954 = F::new(1.0) / t2953;
    let t2955 = t412 * t2954;
    let t2956 = t420 * t420;
    let t2957 = F::new(1.0) / t2956;
    let t2958 = t2931 * t2957;
    let t2961 = F::new(0.12361111111111111111e-1) * t2834;
    let t2966 = t2961 - F::new(0.61805555555555555556e-2) * t2836 - F::new(0.61805555555555555555e-2) * t2843 + F::new(0.18541666666666666667e-1) * t2848 + F::new(0.92708333333333333333e-2) * t2852;
    let t2967 = t2966 * t434;
    (t2950, t2953, t2954, t2955, t2956, t2957, t2958, t2961, t2966, t2967)
}
