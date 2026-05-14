//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1029/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1029<F: Float>(t30783: F, t82431: F, t1945: F, t225: F, t344: F, t6688: F, t6768: F, t23587: F, t8375: F, t23384: F, t30905: F, t30781: F, t6698: F, t1920: F, t2966: F, t8376: F) -> (F, F, F, F, F, F, F) {
    let t113240 = t82431 * t30783;
    let t113243 = t344 * t1945 * t225;
    let t113261 = t6688 * t6768;
    let t113278 = t8375 * t23587;
    let t113286 = t23384 * t30905;
    let t113296 = t6698 * t30781;
    let t113313 = 0.36554090374405031922e-2 * t1920 * t2966 * t8376;
    (t113240, t113243, t113261, t113278, t113286, t113296, t113313)
}
