//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 203/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk203<F: Float>(t307: F, t302: F, t880: F, t906: F, t310: F, t320: F, t315: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t922 = t307 * t307;
    let t923 = F::new(1.0) / t922;
    let t924 = t302 * t923;
    let t926 = F::new(0.516475e0) * t880;
    let t929 = F::new(0.104195e0) * t906;
    let t932 = F::new(1.0) / t310;
    let t936 = F::cast_from(0.92708333333333333333e-2_f64) * t880;
    let t941 = t320 * t320;
    let t942 = F::new(1.0) / t941;
    let t943 = t315 * t942;
    let t945 = F::new(0.301925e0) * t880;
    let t948 = F::new(0.82785e-1) * t906;
    (t922, t923, t924, t926, t929, t932, t936, t941, t942, t943, t945, t948)
}
