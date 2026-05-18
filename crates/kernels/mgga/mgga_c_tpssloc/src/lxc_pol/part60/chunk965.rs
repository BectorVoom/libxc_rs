//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 965/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk965<F: Float>(t1888: F, t32862: F, t86873: F, t118632: F, t23270: F, t25169: F, t5636: F, t22986: F, t30622: F, t5544: F, t118649: F, t118532: F, t32844: F) -> (F, F, F, F, F, F) {
    let t126264 = F::new(0.6579736267392905746e-1) * t1888 * t86873 * t32862;
    let t126278 = F::new(0.3289868133696452873e-1) * t118632;
    let t126286 = F::new(0.9869604401089358619e-1) * t1888 * t23270 * t25169 * t5636;
    let t126290 = F::new(0.3289868133696452873e-1) * t22986 * t23270 * t30622 * t5544;
    let t126291 = F::new(0.15352717957250113407e0) * t118649;
    let t126294 = t118532 * t32844;
    (t126264, t126278, t126286, t126290, t126291, t126294)
}
