//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 365/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk365<F: Float>(t154: F, t244: F, t205: F, t786: F, t792: F, t59: F, t835: F, t116: F, t206: F, t212: F, t2559: F, t222: F, t233: F, t813: F) -> (F, F, F, F, F, F, F, F) {
    let t2570 = t154 * t244;
    let t2571 = t205 * t2570;
    let t2576 = t792 * t786;
    let t2585 = t59 * t835;
    let t2586 = t2585 * t154;
    let t2587 = t206 * t116;
    let t2588 = t2587 * t212;
    let t2590 = 0.83333333333333333332e-3 * t2586 * t2588;
    let t2600 = t2559 * t154;
    let t2602 = 35.0 / 432.0 * t2600 * t222;
    let t2627 = 1.0 / t813 / t233;
    (t2571, t2576, t2586, t2587, t2590, t2600, t2602, t2627)
}
