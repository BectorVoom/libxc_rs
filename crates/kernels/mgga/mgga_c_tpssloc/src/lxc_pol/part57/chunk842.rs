//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 842/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk842<F: Float>(t118480: F, t22986: F, t32814: F, t86873: F, t118472: F, t1484: F, t23270: F, t112899: F, t28267: F, t118821: F, t1527: F, t1888: F, t1880: F, t28263: F, t30663: F, t32862: F) -> (F, F, F, F, F, F, F) {
    let t126226 = 0.15352717957250113407e0 * t118480;
    let t126229 = 0.6579736267392905746e-1 * t22986 * t86873 * t32814;
    let t126233 = 0.6579736267392905746e-1 * t22986 * t23270 * t118472 * t1484;
    let t126240 = 0.6579736267392905746e-1 * t22986 * t112899 * t28267;
    let t126246 = 0.6579736267392905746e-1 * t1888 * t23270 * t118821 * t1527;
    let t126249 = 0.16449340668482264365e-1 * t1880 * t30663 * t28263;
    let t126264 = 0.6579736267392905746e-1 * t1888 * t86873 * t32862;
    (t126226, t126229, t126233, t126240, t126246, t126249, t126264)
}
