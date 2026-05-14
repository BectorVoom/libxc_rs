//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 864/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk864<F: Float>(t22986: F, t23270: F, t30622: F, t5544: F, t118649: F, t118532: F, t32844: F, t16891: F, t232: F, t30714: F, t4180: F, t112792: F, t16839: F, t2632: F, t5617: F, t6605: F, t6612: F) -> (F, F, F, F, F, F, F) {
    let t126290 = 0.3289868133696452873e-1 * t22986 * t23270 * t30622 * t5544;
    let t126291 = 0.15352717957250113407e0 * t118649;
    let t126294 = t118532 * t32844;
    let t126298 = t30714 * t4180 * t16891 * t232;
    let t126302 = t112792 * t4180 * t16839 * t2632;
    let t126306 = t30714 * t4180 * t16839 * t232;
    let t126309 = t6605 * t6612 * t5617;
    (t126290, t126291, t126294, t126298, t126302, t126306, t126309)
}
