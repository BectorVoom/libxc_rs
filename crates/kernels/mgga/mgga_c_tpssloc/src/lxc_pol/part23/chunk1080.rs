//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1080/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1080<F: Float>(t43776: F, t43819: F, t3311: F, t409: F, t3314: F, t3374: F, t3399: F, t440: F, t3355: F, t427: F, t3358: F, t43689: F, t3330: F, t457: F, t625: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t44027 = 0.13388493827160493828e1 * t43776;
    let t44053 = 0.31003950617283950618e1 * t43819;
    let t44073 = t3311 * t3311;
    let t44075 = t409 / t44073;
    let t44076 = t3314 * t3314;
    let t44077 = 1.0 / t44076;
    let t44154 = 1.0 / t3399 / t3374;
    let t44155 = t440 * t44154;
    let t44175 = t3355 * t3355;
    let t44177 = t427 / t44175;
    let t44178 = t3358 * t3358;
    let t44179 = 1.0 / t44178;
    let t44223 = t440 * t43689;
    let t44249 = 0.16979925925925925926e1 * t43776;
    let t44275 = 0.5356037037037037037e1 * t43819;
    let t44320 = 0.17757530864197530864e0 * t43819;
    let t44348 = 0.18467901234567901234e0 * t43819;
    let t44361 = t427 / t3355 / t3330;
    let t44466 = 220.0 / 81.0 * t43776;
    let t44483 = t625 * t457;
    (t44027, t44053, t44075, t44077, t44154, t44155, t44177, t44179, t44223, t44249, t44275, t44320, t44348, t44361, t44466, t44483)
}
