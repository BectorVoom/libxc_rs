//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 200/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk200<F: Float>(t906: F, t310: F, t880: F, t320: F, t315: F, t323: F) -> (F, F, F, F, F, F, F, F, F) {
    let t929 = 0.104195e0 * t906;
    let t932 = 1.0 / t310;
    let t936 = 0.92708333333333333333e-2 * t880;
    let t941 = t320 * t320;
    let t942 = 1.0 / t941;
    let t943 = t315 * t942;
    let t945 = 0.301925e0 * t880;
    let t948 = 0.82785e-1 * t906;
    let t951 = 1.0 / t323;
    (t929, t932, t936, t941, t942, t943, t945, t948, t951)
}
