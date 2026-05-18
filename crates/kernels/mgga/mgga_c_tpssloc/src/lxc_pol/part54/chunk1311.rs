//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1311/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1311<F: Float>(t30714: F, t4240: F, t4250: F, t4191: F, t23270: F, t25038: F, t30622: F, t4255: F, t22986: F, t4119: F, t32814: F, t81651: F, t82074: F) -> (F, F, F, F, F, F) {
    let t118608 = t30714 * t4240;
    let t118610 = t30714 * t4250;
    let t118612 = t30714 * t4191;
    let t118626 = F::new(0.9869604401089358619e-1) * t25038 * t23270 * t30622 * t4255;
    let t118630 = F::new(0.3289868133696452873e-1) * t22986 * t23270 * t30622 * t4119;
    let t118632 = t81651 * t82074 * t32814;
    (t118608, t118610, t118612, t118626, t118630, t118632)
}
