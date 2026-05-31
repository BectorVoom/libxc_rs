//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 229/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk229<F: Float>(t931: F, t932: F, t880: F, t886: F, t324: F, t320: F, t315: F, t906: F, t897: F, t902: F, t910: F, t323: F) -> (F, F, F, F, F, F, F, F) {
    let t933 = t931 * t932;
    let t936 = F::cast_from(0.92708333333333333333e-2_f64) * t880;
    let t938 = -t936 - F::cast_from(0.92708333333333333333e-2_f64) * t886;
    let t939 = t938 * t324;
    let t941 = t320 * t320;
    let t942 = F::cast_from(1.0_f64) / t941;
    let t943 = t315 * t942;
    let t945 = F::cast_from(0.301925e0_f64) * t880;
    let t948 = F::cast_from(0.82785e-1_f64) * t906;
    let t950 = F::cast_from(0.258925e1_f64) * t897 - t945 - F::cast_from(0.301925e0_f64) * t886 + F::cast_from(0.16504875e0_f64) * t902 - t948 - F::cast_from(0.82785e-1_f64) * t910;
    let t951 = F::cast_from(1.0_f64) / t323;
    (t933, t938, t939, t941, t942, t943, t950, t951)
}
