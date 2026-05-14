//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1055/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1055<F: Float>(t22986: F, t30623: F, t86873: F, t1903: F, t254: F, t23168: F, t32789: F, t112873: F, t1527: F, t1888: F, t23270: F, t23185: F, t32862: F, t82074: F, t32863: F, t6579: F) -> (F, F, F, F, F, F) {
    let t118639 = 0.3289868133696452873e-1 * t22986 * t86873 * t30623;
    let t118640 = t1903 * t254;
    let t118649 = t23168 * t32789;
    let t118650 = 0.76763589786250567037e-1 * t118649;
    let t118654 = 0.3289868133696452873e-1 * t1888 * t23270 * t112873 * t1527;
    let t118661 = t23185 * t82074 * t32862;
    let t118662 = 0.16449340668482264365e-1 * t118661;
    let t118663 = t6579 * t32863;
    (t118639, t118640, t118650, t118654, t118662, t118663)
}
