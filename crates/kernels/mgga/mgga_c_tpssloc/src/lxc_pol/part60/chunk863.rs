//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 863/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk863<F: Float>(t118472: F, t1484: F, t22986: F, t23270: F, t112899: F, t28267: F, t118821: F, t1527: F, t1888: F, t1880: F, t28263: F, t30663: F, t32862: F, t86873: F, t118632: F, t25169: F, t5636: F) -> (F, F, F, F, F, F, F) {
    let t126233 = 0.6579736267392905746e-1 * t22986 * t23270 * t118472 * t1484;
    let t126240 = 0.6579736267392905746e-1 * t22986 * t112899 * t28267;
    let t126246 = 0.6579736267392905746e-1 * t1888 * t23270 * t118821 * t1527;
    let t126249 = 0.16449340668482264365e-1 * t1880 * t30663 * t28263;
    let t126264 = 0.6579736267392905746e-1 * t1888 * t86873 * t32862;
    let t126278 = 0.3289868133696452873e-1 * t118632;
    let t126286 = 0.9869604401089358619e-1 * t1888 * t23270 * t25169 * t5636;
    (t126233, t126240, t126246, t126249, t126264, t126278, t126286)
}
